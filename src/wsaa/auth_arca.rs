use chrono::{DateTime, Duration, FixedOffset, NaiveDateTime, Utc};
use reqwest::{header::CONTENT_TYPE, Client};

use crate::{crypto::sign_cms::sign_cms, types::{enums::Webservice, errors::{ErrType, SoapFault}}, wsaa::url::{URL_HOMO, URL_PROD}, xml_utils::get_xml_tag};

use super::get_token::TokenArca;

pub async fn auth_arca(
	webservice		: Webservice,
	cert_contents	: &Vec<u8>,
	key_contents	: &Vec<u8>,
	req_cli				: &Client,
	es_prod				: bool,
	cuit					: i64,
) -> Result<TokenArca, ErrType> {
	let url = if es_prod {URL_PROD} else {URL_HOMO};

	let tz:FixedOffset = FixedOffset::west_opt(3600*3).unwrap();
	let req_date = Utc::now() - Duration::minutes(5);
	let exp_date = req_date + Duration::hours(23);
	let login_ticket = login_ticket_request_xml(webservice, req_date, exp_date);
	
	let signed_ticket = sign_cms(
		cert_contents,
		key_contents,
		login_ticket.as_str());
	let request_xml = make_xml(&signed_ticket);

	let response = req_cli.post(url)
		.header(CONTENT_TYPE, "text/xml")
		.header("SOAPAction", "")
		.body(request_xml)
		.timeout(std::time::Duration::from_secs(60))
		.send().await?
		.text().await?;

	if response.contains("<faultcode") {
		if response.contains("ns1:coe.alreadyAuthenticated") {
			return Err(SoapFault::new(
				"alreadyAuthenticated",
				"Estas renovando el login muy rapido y ARCA no quiso darte uno nuevo. Intenta en unos minutos"
			).into());
		} else {
			dbg!(&response);
			return Err(SoapFault::from_xml(&response).into());
		}
	};
	

	let expir_str = get_xml_tag(&response, "expirationTime")
		.ok_or(SoapFault::new("parseError","No se encontro expirationTime en la respuesta de afip"))?;
	let token= get_xml_tag(&response, "token")
		.ok_or(SoapFault::new("parseError","No se encontro token en la respuesta de afip"))?;
	let sign = get_xml_tag(&response, "sign")
		.ok_or(SoapFault::new("parseError","No se encontro sign en la respuesta de afip"))?;
	let expir = NaiveDateTime::parse_from_str(&expir_str,"%Y-%m-%dT%H:%M:%S%.f%:z").unwrap();
	//dbg!(&expir);
	let expir = expir.and_local_timezone(tz).unwrap().to_utc();
	//dbg!(&expir);
	
	return Ok(TokenArca{cuit, token, sign, expir});
}


fn make_xml(signed_request:&str) -> String {
	return format!(
		r#"<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:wsaa="http://wsaa.view.sua.dvadac.desein.afip.gov">
			<soapenv:Header/>
			<soapenv:Body>
					<wsaa:loginCms>
						<wsaa:in0>{signed_request}</wsaa:in0>
					</wsaa:loginCms>
			</soapenv:Body>
		</soapenv:Envelope>"#
	)
}


fn login_ticket_request_xml(
	webservice:Webservice,req_date: DateTime<Utc>,exp_date: DateTime<Utc>
) -> String {

	let webservice = webservice.to_string();
	let gen_time = req_date.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
	let exp_time = exp_date.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
	let uniqueid = req_date.timestamp().to_string();

	let login_ticket_request_xml = format!(
	r#"<loginTicketRequest version="1.0">
		<header>
			<uniqueId>{uniqueid}</uniqueId>
			<generationTime>{gen_time}</generationTime>
			<expirationTime>{exp_time}</expirationTime>
		</header>
		<service>{webservice}</service>
	</loginTicketRequest>"#
	);
	return login_ticket_request_xml;
}




