use std::{sync::Arc, time::Duration};

use reqwest::{
    Client,
    header::{ACCEPT_CHARSET, CONTENT_TYPE},
};
use tracing::{debug, error, info, warn};

use crate::{
    types::{enums::Webservice, errors::ErrType},
    wsaa::get_token::{CertKeyPair, ServiceId, TokenArca, get_token},
    wsfev1::{
        fe_cae_solicitar::types::{CodeMsgParse, Wsfev1Obs},
        fe_comp_ultimo_autorizado::{
            types::{FeCompUltimoAutorizadoParse, UltimoAutorizadoRetorno},
            xml_make::xml_make,
        },
        url::{WSFEV1_URL_HOMO, WSFEV1_URL_PROD},
    },
    xml_utils::get_xml_tag,
};

/// Consulta el ultimo comprobante autorizado para un punto de venta y tipo de comprobante
/// dados. Util, por ejemplo, ante un rechazo con codigo 10016 ("El numero o fecha del
/// comprobante no se corresponde con el proximo a autorizar"): el proximo numero valido
/// es `cbte_nro + 1`.
pub async fn fecomp_ultimo_autorizado<Fc>(
    token_map: Arc<dashmap::DashMap<ServiceId, TokenArca>>,
    tenant_id: i64,
    es_prod: bool,
    req_cli: &Client,
    pto_vta: i64,
    cbte_tipo: i64,
    cert_key_getter: Fc,
) -> Result<UltimoAutorizadoRetorno, ErrType>
where
    Fc: AsyncFnMut() -> Option<CertKeyPair>,
{
    info!(
        pto_vta,
        cbte_tipo, tenant_id, es_prod, "Consultando FECompUltimoAutorizado"
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
        error!(pto_vta, cbte_tipo, tenant_id, error = ?e, "Error obteniendo token para wsfev1")
    })?;

    let send_xml = xml_make(pto_vta, cbte_tipo, auth_xml);
    let sent_xml = xml_make(pto_vta, cbte_tipo, "<!-- auth omitido -->".to_owned());

    debug!(
        pto_vta,
        cbte_tipo,
        ?sent_xml,
        "Request XML de FECompUltimoAutorizado generado"
    );

    let req = req_cli
        .post(url)
        .header(CONTENT_TYPE, "application/soap+xml; charset=utf-8")
        .header(ACCEPT_CHARSET, "utf-8")
        .body(send_xml)
        .timeout(Duration::from_secs(60));

    let res = req.send().await.inspect_err(
        |e| error!(pto_vta, cbte_tipo, error = ?e, "Error de red al llamar a FECompUltimoAutorizado"),
    )?;

    let received_xml = res.text().await.inspect_err(
        |e| error!(pto_vta, cbte_tipo, error = ?e, "Error leyendo el cuerpo de la respuesta de FECompUltimoAutorizado"),
    )?;

    debug!(
        pto_vta,
        cbte_tipo,
        ?received_xml,
        "Respuesta de FECompUltimoAutorizado recibida"
    );

    if received_xml.contains("<soap:Fault>") {
        error!(pto_vta, cbte_tipo, respuesta = %received_xml, "FECompUltimoAutorizado devolvio un SOAP Fault");
        return Err(crate::types::errors::SoapFault::from_xml(&received_xml).into());
    }

    let xml_recortado =
        get_xml_tag(&received_xml, "FECompUltimoAutorizadoResult").ok_or_else(|| {
            error!(pto_vta, cbte_tipo, respuesta = %received_xml, "No se encontro el tag 'FECompUltimoAutorizadoResult'");
            "No se encontro el tag 'FECompUltimoAutorizadoResult'".to_owned()
        })?;
    let xml_recortado = format!("<r>{xml_recortado}</r>");

    let parsed: FeCompUltimoAutorizadoParse =
        quick_xml::de::from_str(&xml_recortado).map_err(|err| {
            error!(pto_vta, cbte_tipo, error = ?err, xml_recortado = %xml_recortado, "Error parseando la respuesta de FECompUltimoAutorizado");
            err.to_string()
        })?;

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
    obs.extend(
        parsed
            .events
            .map(|e| e.items)
            .unwrap_or_default()
            .into_iter()
            .map(to_obs),
    );

    if !obs.is_empty() {
        warn!(
            pto_vta,
            cbte_tipo,
            ?obs,
            "Observaciones/Eventos/Errores encontrados en la respuesta"
        );
    }

    let Some(cbte_nro) = parsed.cbte_nro else {
        error!(pto_vta, cbte_tipo, respuesta = %received_xml, "No se encontro CbteNro en la respuesta");
        return Err(
            crate::types::errors::SoapFault::new("", "No se encontro CbteNro en la respuesta")
                .into(),
        );
    };

    Ok(UltimoAutorizadoRetorno {
        pto_vta: parsed.pto_vta.map(i64::from).unwrap_or(pto_vta),
        cbte_tipo: parsed.cbte_tipo.map(i64::from).unwrap_or(cbte_tipo),
        cbte_nro: cbte_nro.into(),
        obs,
        sent_xml,
        received_xml,
    })
}

fn token_parser(cuit: i64, token: &str, sign: &str) -> String {
    format!(
r#"<ar:Auth>\
	<ar:Token>{token}</ar:Token>\
	<ar:Sign>{sign}</ar:Sign>\
	<ar:Cuit>{cuit}</ar:Cuit>\
</ar:Auth>"#
    )
}