use std::time::Instant;

use reqwest::{header::CONTENT_TYPE, Client};

use crate::{types::FEDummyResult, wsfexv1::url::{WSFEXV1_URL_HOMO, WSFEXV1_URL_PROD}, xml_utils::get_xml_tag};

/// Consulta el metodo FEDummy para saber si el servicio esta corriendo o no
pub async fn service_status(req_cli : &Client, es_prod:bool) -> FEDummyResult {
	let mut retorno = FEDummyResult {
		status     : reqwest::StatusCode::BAD_REQUEST,
		app_server : false,
		db_server  : false,
		auth_server: false,
		milis_respuesta : 0,
	};

	let url = if es_prod {WSFEXV1_URL_PROD} else {WSFEXV1_URL_HOMO};
	
	let send_xml = 
r#"<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/">
 <soapenv:Body>
  <tns:FEXDummy xmlns:tns="http://ar.gov.afip.dif.fexv1/"></tns:FEXDummy>
 </soapenv:Body>
</soapenv:Envelope>"#;

	let start = Instant::now();
	let req = req_cli.post(url)
	.header(CONTENT_TYPE, "text/xml")
	.body(send_xml)
	.send().await;

	retorno.milis_respuesta = start.elapsed().as_millis();
	

	match req {
		Ok(online) => {
			retorno.status = online.status();

			let txt = online.text().await.unwrap();
			retorno.app_server  = get_xml_tag(&txt, "AppServer" ).map(|x| x.to_uppercase().trim() == "OK").unwrap_or(false);
			retorno.db_server   = get_xml_tag(&txt, "DbServer"  ).map(|x| x.to_uppercase().trim() == "OK").unwrap_or(false);
			retorno.auth_server = get_xml_tag(&txt, "AuthServer").map(|x| x.to_uppercase().trim() == "OK").unwrap_or(false);
		},
		Err(er) => {
			match er.status() {
				Some(status) => {retorno.status = status},
				None => {
					if er.is_connect() {
						dbg!(&er);
						retorno.status = reqwest::StatusCode::SERVICE_UNAVAILABLE;
					} else if er.is_request() {
						dbg!(&er);
						retorno.status = reqwest::StatusCode::BAD_REQUEST;
					} else if er.is_timeout() {
						dbg!(&er);
						retorno.status = reqwest::StatusCode::REQUEST_TIMEOUT;
					} else {
						dbg!(&er);
						retorno.status = reqwest::StatusCode::INTERNAL_SERVER_ERROR;
					}
				},
			}
		},
	}

	return retorno;
}

