use std::{sync::Arc, time::Duration};

use futures::future;
use reqwest::{
    Client,
    header::{ACCEPT_CHARSET, CONTENT_TYPE},
};

use crate::{
    types::{
        enums::Webservice,
        errors::{ErrType, SoapFault},
    },
    ws_sr_padron_a13::{get_by_cuit::get_persona_v2, types::PersonaCuitRetorno},
    wsaa::get_token::{CertKeyPair, ServiceId, TokenArca, get_token},
    xml_utils::get_xml_vec,
};

use super::url::{WS_SR_PADRON_A13_URL_HOMO, WS_SR_PADRON_A13_URL_PROD};

/// Consulta el metodo getIdPersonaListByDocumento para obtener los CUIT/CUIL validos para el DNI recibido
/// Y luego consulta get_persona_v2 para cada uno
pub async fn get_by_dni<Fc>(
    token_map: Arc<dashmap::DashMap<ServiceId, TokenArca>>,
    tenant_id: i64,
    es_prod: bool,
    req_cli: &Client,
    dni: i64,
    cert_key_getter: Fc,
) -> Result<Vec<PersonaCuitRetorno>, ErrType>
where
    Fc: AsyncFnMut() -> Option<CertKeyPair>,
{
    let url = if es_prod {
        WS_SR_PADRON_A13_URL_PROD
    } else {
        WS_SR_PADRON_A13_URL_HOMO
    };
    let key = ServiceId {
        tenant_id,
        webservice: Webservice::WsSrPadronA13,
    };
    let auth_xml = get_token(
        token_map,
        key,
        es_prod,
        req_cli,
        cert_key_getter,
        token_parser,
    )
    .await?;

    let send_xml = xml_make(dni, &auth_xml);

    let req = req_cli
        .post(url)
        .header(CONTENT_TYPE, "application/soap+xml; charset=utf-8") //Hay que aclarar el charset porque arca miente y manda windows-1252 diciendo que es utf-8
        .header(ACCEPT_CHARSET, "utf-8")
        .body(send_xml.clone())
        .timeout(Duration::from_secs(60));

    let res = req.send().await?;

    let text = res.text().await?;
    dbg!(&text);

    if text.contains("<soap:Fault>") {
        return Err(SoapFault::from_xml(&text).into());
    }

    let list = get_xml_vec(&text, "idPersona")
        .into_iter()
        .filter_map(|x| x.parse::<i64>().ok())
        .collect::<Vec<_>>();

    let futures = list
        .into_iter()
        .map(|cuit| get_persona_v2(url, req_cli, cuit, &auth_xml));

    future::try_join_all(futures).await
}

fn token_parser(cuit: i64, token: &str, sign: &str) -> String {
    format!(
        r#"<token>{token}</token>
         <sign>{sign}</sign>
         <cuitRepresentada>{cuit}</cuitRepresentada>"#
    )
}

fn xml_make(dni: i64, auth_xml: &str) -> String {
    format!(
        r#"<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:a13="http://a13.soap.ws.server.puc.sr/">
   <soapenv:Header/>
   <soapenv:Body>
      <a13:getIdPersonaListByDocumento>
				{auth_xml}
         <documento>{dni}</documento>
      </a13:getIdPersonaListByDocumento>
   </soapenv:Body>
</soapenv:Envelope>"#
    )
}
