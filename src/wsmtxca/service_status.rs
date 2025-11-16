use std::time::{Duration, Instant};

use reqwest::{header::CONTENT_TYPE, Client};

use crate::{types::FEDummyResult, wsmtxca::url::{WSMTXCA_URL_HOMO, WSMTXCA_URL_PROD}, xml_utils::get_xml_tag};

/// Consulta el metodo FEDummy para saber si el servicio esta corriendo o no
pub async fn service_status(req_cli : &Client, es_prod:bool, timeout:Option<Duration>) -> FEDummyResult {
	let mut retorno = FEDummyResult {
		status     : reqwest::StatusCode::BAD_REQUEST,
		app_server : false,
		db_server  : false,
		auth_server: false,
		milis_respuesta : 0,
	};

	let url = if es_prod {WSMTXCA_URL_PROD} else {WSMTXCA_URL_HOMO};
	
	let send_xml = 
r#"<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/">
 <soapenv:Body>
  <tns:dummy xmlns:tns="http://impl.service.wsmtxca.afip.gov.ar/service/"></tns:dummy>
 </soapenv:Body>
</soapenv:Envelope>"#;

	let start = Instant::now();
	let req = req_cli.post(url)
	.header(CONTENT_TYPE, "text/xml")
	.body(send_xml)
	.timeout(timeout.unwrap_or(Duration::from_secs(30)))
	.send().await;

	retorno.milis_respuesta = start.elapsed().as_millis();
	

	match req {
		Ok(online) => {
			retorno.status = online.status();

			let txt = online.text().await.unwrap();
			retorno.app_server  = get_xml_tag(&txt, "appserver" ).map(|x| x.to_uppercase().trim() == "OK").unwrap_or(false);
			retorno.db_server   = get_xml_tag(&txt, "dbserver"  ).map(|x| x.to_uppercase().trim() == "OK").unwrap_or(false);
			retorno.auth_server = get_xml_tag(&txt, "authserver").map(|x| x.to_uppercase().trim() == "OK").unwrap_or(false);
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

