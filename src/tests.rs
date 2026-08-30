use std::{fs, sync::{Arc, OnceLock}};

use crate::{CertKeyPair, ServiceId, TokenArca};

static TOKEN_MAP: OnceLock<Arc<dashmap::DashMap<ServiceId, TokenArca>>> = OnceLock::new();

pub fn test_token_map() -> Arc<dashmap::DashMap<ServiceId, TokenArca>> {
    TOKEN_MAP
        .get_or_init(|| Arc::new(dashmap::DashMap::new()))
        .clone()
}

pub async fn test_cert_key_getter() -> Option<CertKeyPair> {
	let cert_contents: Vec<u8> = fs::read("cert_test.pem").expect("error on handling cert file");
	let key_contents: Vec<u8> = fs::read("key_test.key").expect("error on handling key file");
	let cuit: i64 = 20398305923;

	Some(CertKeyPair { cuit, cert_contents, key_contents })
}