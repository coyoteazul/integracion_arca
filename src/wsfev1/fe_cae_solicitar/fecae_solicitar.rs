use std::{sync::Arc, time::Duration};

use chrono::{Days, NaiveDate, Utc};
use reqwest::{
    Client,
    header::{ACCEPT_CHARSET, CONTENT_TYPE},
};
use tracing::{debug, error, info, warn};

use crate::{
    types::{enums::Webservice, errors::ErrType},
    wsaa::get_token::{CertKeyPair, ServiceId, TokenArca, get_token},
    wsfev1::{
        fe_cae_solicitar::{
            types::{
                CodeMsgParse, Comprobante, FecaeResponseParse, FecaeRetorno, Wsfev1Obs, Wsfev1Ok,
                Wsfev1Rechazo,
            },
            xml_make::xml_make,
        },
        url::{WSFEV1_URL_HOMO, WSFEV1_URL_PROD},
    },
    xml_utils::get_xml_tag,
};

/// Genera el request, lo envia y parsea la respuesta en un solo paso.
/// `cert_key_getter` solo se llama si es necesario renovar el token.
pub async fn fecae_solicitar<Fc>(
    token_map: Arc<dashmap::DashMap<ServiceId, TokenArca>>,
    tenant_id: i64,
    es_prod: bool,
    req_cli: &Client,
    comprobante: &Comprobante,
    cert_key_getter: Fc,
) -> Result<FecaeRetorno, ErrType>
where
    Fc: AsyncFnMut() -> Option<CertKeyPair>,
{
    info!(
        id_factura = comprobante.id_factura,
        punto_venta = comprobante.cabezal.punto_venta,
        tenant_id,
        es_prod,
        "Solicitando FECAESolicitar"
    );

    let url = if es_prod {
        WSFEV1_URL_PROD
    } else {
        WSFEV1_URL_HOMO
    };
    let key = ServiceId {
        tenant_id,
        webservice: Webservice::Wsfev1,
    };
    let auth_xml = get_token(
        token_map,
        key,
        es_prod,
        req_cli,
        cert_key_getter,
        token_parser,
    )
    .await
    .inspect_err(|e| {
        error!(
            id_factura = comprobante.id_factura,
            tenant_id,
            error = ?e,
            "Error obteniendo token para wsfev1"
        )
    })?;

    let send_xml = xml_make(comprobante, auth_xml);
    // Version para loguear/guardar: mismo request, sin datos de autenticacion.
    let expugned_xml = xml_make(comprobante, "<!-- auth omitido -->".to_owned());

    debug!(
        id_factura = comprobante.id_factura,
        ?expugned_xml,
        "Request XML de FECAESolicitar generado"
    );

    let req = req_cli
        .post(url)
        .header(CONTENT_TYPE, "application/soap+xml; charset=utf-8") //Hay que aclarar el charset porque arca miente y manda windows-1252 diciendo que es utf-8
        .header(ACCEPT_CHARSET, "utf-8")
        .body(send_xml)
        .timeout(Duration::from_secs(60));

    let res = req.send().await.inspect_err(
        |e| error!(id_factura = comprobante.id_factura, error = ?e, "Error de red al llamar a FECAESolicitar"),
    )?;

    let received_xml = res.text().await.inspect_err(
        |e| error!(id_factura = comprobante.id_factura, error = ?e, "Error leyendo el cuerpo de la respuesta de FECAESolicitar"),
    )?;

		debug!(
        id_factura = comprobante.id_factura,
        ?received_xml,
        "Request XML de FECAESolicitar recibido"
    );

    let resultado = parse_response(&received_xml, comprobante.id_factura)?;

    Ok(FecaeRetorno {
        resultado,
        sent_xml: expugned_xml,
        received_xml,
    })
}

fn token_parser(cuit: i64, token: &str, sign: &str) -> String {
    format!(
        r#"<ar:Auth>
	<ar:Token>{token}</ar:Token>
	<ar:Sign>{sign}</ar:Sign>
	<ar:Cuit>{cuit}</ar:Cuit>
</ar:Auth>"#
    )
}

/// Parsea el SOAP Fault por separado (transporte/infraestructura); todo lo demas
/// -incluido un rechazo de negocio, Resultado="R"- se modela como Ok(Err(Wsfev1Rechazo)).
fn parse_response(
    respuesta: &str,
    id_factura: i64,
) -> Result<Result<Wsfev1Ok, Wsfev1Rechazo>, ErrType> {
    if respuesta.contains("<soap:Fault>") {
        error!(id_factura, respuesta = %respuesta, "FECAESolicitar devolvio un SOAP Fault");
        return Err(crate::types::errors::SoapFault::from_xml(respuesta).into());
    }

    let xml_recortado = get_xml_tag(respuesta, "FECAESolicitarResult").ok_or_else(|| {
        error!(id_factura, respuesta = %respuesta, "No se encontro el tag 'FECAESolicitarResult'");
        "No se encontro el tag 'FECAESolicitarResult'".to_owned()
    })?;
    let xml_recortado = format!("<r>{xml_recortado}</r>");

    let parsed: FecaeResponseParse = quick_xml::de::from_str(&xml_recortado).map_err(|err| {
        error!(id_factura, error = ?err, xml_recortado = %xml_recortado, "Error parseando la respuesta de FECAESolicitar");
        err.to_string()
    })?;

		debug!(
        id_factura,
        ?parsed,
        "Request XML de FECAESolicitar parseado"
    );

    let to_obs = |c: CodeMsgParse| Wsfev1Obs {
        code: c.code.map(|x| x.to_string()).unwrap_or_default(),
        msg: c.msg.unwrap_or_default(),
    };

    let mut obs: Vec<Wsfev1Obs> = parsed
        .errors
        .map(|e| e.items)
        .unwrap_or_default()
        .into_iter()
        .map(to_obs)
        .collect();

    let Some(det) = parsed.det.items.into_iter().next() else {
        error!(id_factura, respuesta = %respuesta, "No se encontro FEDetResponse en la respuesta");
        return Err(crate::types::errors::SoapFault::new(
            "",
            "No se encontro informacion de detalle en la respuesta",
        )
        .into());
    };

    obs.extend(det.observaciones.items.into_iter().map(to_obs));

		obs.extend(parsed.events.map_or(vec![], |e| e.items.into_iter().map(to_obs).collect()));

    if !obs.is_empty() {
        warn!(
            id_factura,
            ?obs,
            "Observaciones/Eventos/Errores encontrados en la respuesta"
        );
    }		

    // Usamos el Resultado del detalle (no el de FeCabResp): con CantReg=1 siempre
    // deberian coincidir, pero el del detalle es el que corresponde semanticamente
    // a ESTE comprobante.
    if det.resultado.as_deref() == Some("R") {
        warn!(id_factura, ?obs, "Documento rechazado por ARCA");
        return Ok(Err(Wsfev1Rechazo { obs }));
    }

    match (det.cae, det.cae_fch_vto) {
        (Some(cae), Some(vcto_str)) => {
            let vcto = NaiveDate::parse_from_str(&vcto_str, "%Y%m%d").unwrap_or_else(|err| {
                warn!(
                    id_factura,
                    error = ?err,
                    vcto_str = %vcto_str,
                    "No se pudo parsear la fecha de vencimiento del CAE. Se asume que es 10 dias mayor a hoy"
                );
                Utc::now().checked_add_days(Days::new(10)).unwrap().date_naive()
            });
            debug!(id_factura, cae = %cae, vcto = %vcto, "CAE obtenido correctamente");
            Ok(Ok(Wsfev1Ok { cae, vcto, obs }))
        }
        _ => {
            error!(id_factura, respuesta = %respuesta, "El documento fue aprobado pero no se encontro el CAE o su fecha de vencimiento");
            Err(crate::types::errors::SoapFault::new(
                "",
                "El documento fue aprobado pero no se encontro el CAE o su fecha de vencimiento",
            )
            .into())
        }
    }
}
