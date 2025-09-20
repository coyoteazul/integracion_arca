use std::time::Instant;

use reqwest::{header::CONTENT_TYPE, Client};

use crate::{types::FEDummyResult, wsfev1::url::{WSFEV1_URL_HOMO, WSFEV1_URL_PROD}, xml_utils::get_xml_tag};

/// Consulta el metodo FEDummy para saber si el servicio esta corriendo o no
pub async fn service_status(req_cli : &Client, es_prod:bool) -> FEDummyResult {
	let mut retorno = FEDummyResult {
		status     : reqwest::StatusCode::BAD_REQUEST,
		app_server : false,
		db_server  : false,
		auth_server: false,
		milis_respuesta : 0,
	};

	let url = if es_prod {WSFEV1_URL_PROD} else {WSFEV1_URL_HOMO};
	
	let send_xml = 
r#"<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/">
 <soapenv:Body>
  <tns:FEDummy xmlns:tns="http://ar.gov.afip.dif.FEV1/"/>
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
			retorno.status = er.status().unwrap();
			dbg!(&er);
		},
	}

	return retorno;
}

