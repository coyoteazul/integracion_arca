use std::sync::Arc;

use chrono::{DateTime, Duration, Utc};
use reqwest::Client;

use crate::{types::{enums::Webservice, errors::{ErrType, SoapFault}}, wsaa::auth_arca::auth_arca};



///Busca el token de dentro de un DashMap. Si no lo encuentra o si esta expirado, lo renueva.
/// 
/// `cert_key_getter` solo se llama en caso de que sea necesario renovar el token. 
/// Debe devoler el contenido del certificado y la llave privada, ya sea leyendolo de un archivo o de la base de datos.
/// 
/// `token_parser` Recibe el token y el sign y deberia devolver un string formateado. Es el resultado final de la funcion
pub async fn get_token<'a, Fc>(
	token_map				: Arc<dashmap::DashMap<ServiceId, TokenArca>>,
	key							: ServiceId,
	es_prod					: bool,
	req_cli					: &Client,
	mut cert_key_getter	: Fc,
	token_parser  	: fn(i64, &str, &str) -> String
)-> Result<String, ErrType> 
where 
	Fc: AsyncFnMut() -> Option<CertKeyPair>,
{
	if let Some(rf) = token_map.get(&key) {
		let current_time = dbg!(Utc::now()+Duration::minutes(15));
		if rf.value().expir > current_time {
			return Ok(token_parser(rf.cuit, &rf.token, &rf.sign));
		}
	};

	let CertKeyPair {cuit, cert_contents, key_contents} = cert_key_getter().await.ok_or(SoapFault::new("db", "No se encontro el par de Certificado y Key"))?;
	let value = auth_arca(key.webservice, &cert_contents, &key_contents, req_cli, es_prod, cuit).await?;
	let retorno = token_parser(cuit, &value.token, &value.sign);
	token_map.insert(key, value);
	return Ok(retorno);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ServiceId {
	pub(crate) tenant_id: i64,
	pub(crate) webservice: Webservice,
}

#[derive(Debug)]
pub struct TokenArca {
	pub(super) cuit			: i64,
	pub(super) token		: String,
	pub(super) sign 		: String,
	pub(super) expir		: DateTime<Utc>,
}

pub struct CertKeyPair {
	pub cuit          : i64,
	pub cert_contents	: Vec<u8>,
	pub key_contents 	: Vec<u8>,
}