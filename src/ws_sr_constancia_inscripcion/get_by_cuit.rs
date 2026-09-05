use std::{sync::Arc, time::Duration};

use reqwest::{
    Client,
    header::{ACCEPT_CHARSET, CONTENT_TYPE},
};
use tracing::{error, info, trace};

use crate::{
    types::{
        enums::Webservice,
        errors::{ErrType, SoapFault},
    },
    ws_sr_constancia_inscripcion::types::{
        ConstanciaInscripcion, ConstanciaInscripcionRetorno, PersonaA5Parse,
    },
    wsaa::get_token::{CertKeyPair, ServiceId, TokenArca, get_token},
    xml_utils::get_xml_tag,
};

use super::url::{WS_SR_CONSTANCIA_INSCRIPCION_URL_HOMO, WS_SR_CONSTANCIA_INSCRIPCION_URL_PROD};

/// Consulta el metodo getPersona
pub async fn get_by_cuit<Fc>(
    token_map: Arc<dashmap::DashMap<ServiceId, TokenArca>>,
    tenant_id: i64,
    es_prod: bool,
    req_cli: &Client,
    cuit: i64,
    cert_key_getter: Fc,
) -> Result<ConstanciaInscripcionRetorno, ErrType>
where
    Fc: AsyncFnMut() -> Option<CertKeyPair>,
{
    info!(cuit, tenant_id, es_prod, "Consultando getPersona por CUIT");

    let url = if es_prod {
        WS_SR_CONSTANCIA_INSCRIPCION_URL_PROD
    } else {
        WS_SR_CONSTANCIA_INSCRIPCION_URL_HOMO
    };
    let key = ServiceId {
        tenant_id,
        webservice: Webservice::WsSrConstanciaInscripcion,
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
    .inspect_err(
        |e| error!(cuit, tenant_id, error = ?e, "Error obteniendo token para getPersona"),
    )?;

    get_persona(url, req_cli, cuit, &auth_xml).await
}

/// Metodo interno para llamar a getPersona sin recalcular tokens
pub(crate) async fn get_persona(
    url: &str,
    req_cli: &Client,
    cuit: i64,
    auth_xml: &str,
) -> Result<ConstanciaInscripcionRetorno, ErrType> {
    let send_xml = xml_make(cuit, auth_xml.to_string());

    let req = req_cli
        .post(url)
        .header(CONTENT_TYPE, "application/soap+xml; charset=utf-8") //Hay que aclarar el charset porque arca miente y manda windows-1252 diciendo que es utf-8
        .header(ACCEPT_CHARSET, "utf-8")
        .body(send_xml.clone())
        .timeout(Duration::from_secs(60));

    let res = req
        .send()
        .await
        .inspect_err(|e| error!(cuit, error = ?e, "Error de red al llamar a getPersona"))?;

    let answer_xml = res.text().await.inspect_err(
        |e| error!(cuit, error = ?e, "Error leyendo el cuerpo de la respuesta de getPersona"),
    )?;

    if answer_xml.contains("<soap:Fault>") {
        error!(cuit, respuesta = %answer_xml, "getPersona devolvio un SOAP Fault");
        return Err(SoapFault::from_xml(&answer_xml).into());
    }
    let xml_recortado = get_xml_tag(&answer_xml, "personaReturn").ok_or_else(|| {
        error!(cuit, respuesta = %answer_xml, "No se encontro el tag 'personaReturn' en la respuesta");
        "No se encontro el tag 'personaReturn'".to_owned()
    })?;
    let xml_recortado = format!("<personaReturn>{xml_recortado}</personaReturn>");
    trace!(cuit, xml_recortado = %xml_recortado, "XML de persona recortado");

    match quick_xml::de::from_str::<PersonaA5Parse>(&xml_recortado) {
        Ok(persona) => {
            let parsed: PersonaA5Parse = persona.into();
            info!(cuit, "Persona obtenida correctamente via getPersona");
            Ok(ConstanciaInscripcionRetorno {
                parsed: ConstanciaInscripcion::from(parsed),
                answer_xml,
            })
        }
        Err(err) => {
            error!(cuit, error = ?err, xml_recortado = %xml_recortado, "Error parseando la respuesta de getPersona");
            Err(err.to_string().into())
        }
    }
}

fn token_parser(cuit: i64, token: &str, sign: &str) -> String {
    format!(
        r#"<token>{token}</token>
         <sign>{sign}</sign>
         <cuitRepresentada>{cuit}</cuitRepresentada>"#
    )
}

fn xml_make(cuit: i64, auth_xml: String) -> String {
    format!(
        r#"<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:a5="http://a5.soap.ws.server.puc.sr/">
<soapenv:Header/>
  <soapenv:Body>
    <a5:getPersona_v2>
      {auth_xml}
      <idPersona>{cuit}</idPersona>
    </a5:getPersona_v2>
  </soapenv:Body>
</soapenv:Envelope>"#
    )
}
