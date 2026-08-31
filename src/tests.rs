use std::{
    fs,
    sync::{Arc, Once, OnceLock},
};

use crate::{CertKeyPair, ServiceId, TokenArca};

static TOKEN_MAP: OnceLock<Arc<dashmap::DashMap<ServiceId, TokenArca>>> = OnceLock::new();
static TRACING_INIT: Once = Once::new();

pub fn test_token_map() -> Arc<dashmap::DashMap<ServiceId, TokenArca>> {
    TOKEN_MAP
        .get_or_init(|| Arc::new(dashmap::DashMap::new()))
        .clone()
}

pub async fn test_cert_key_getter() -> Option<CertKeyPair> {
    let cert_contents: Vec<u8> = fs::read("cert_test.pem").expect("error on handling cert file");
    let key_contents: Vec<u8> = fs::read("key_test.key").expect("error on handling key file");
    let cuit: i64 = 20398305923;

    Some(CertKeyPair {
        cuit,
        cert_contents,
        key_contents,
    })
}

/// Inicializa un subscriber de tracing que escribe hacia el test writer de libtest,
/// asi los logs aparecen intercalados con el test que los genero sin necesitar `--nocapture`.
/// Protegido con `Once` porque cada #[tokio::test] corre en su propio hilo y llamarlo
/// mas de una vez haria panic (`set_global_default` solo se puede llamar una vez).
pub fn init_tracing() {
    TRACING_INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_test_writer()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("debug")),
            )
            .init();
    });
}
