use std::{fs, path::PathBuf, sync::Arc};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{
    tests::{init_tracing, test_cert_key_getter},
    types::enums::Webservice,
    wsaa::get_token::{ServiceId, TokenArca},
    wsfev1::fe_comp_ultimo_autorizado::fecomp_ultimo_autorizado::fecomp_ultimo_autorizado,
};

// Mismo esquema (y, por default, el mismo archivo) que fe_cae_solicitar/tests.rs: ambos
// modulos comparten la ServiceId (tenant_id, Webservice::Wsfev1), asi que corriendo
// cualquiera de los dos primero ya deja el token cacheado para el otro.

#[derive(Serialize, Deserialize)]
struct CachedToken {
    cuit: i64,
    token: String,
    sign: String,
    expir: DateTime<Utc>,
}

fn cache_path() -> PathBuf {
    std::env::var("ARCA_TEST_TOKEN_CACHE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir().join("arca_wsfev1_test_token.json"))
}

fn test_token_map_with_cache(tenant_id: i64) -> Arc<dashmap::DashMap<ServiceId, TokenArca>> {
    let map = Arc::new(dashmap::DashMap::new());

    if let Ok(contents) = fs::read_to_string(cache_path()) {
        if let Ok(cached) = serde_json::from_str::<CachedToken>(&contents) {
            if cached.expir > Utc::now() + chrono::Duration::minutes(20) {
                let key = ServiceId {
                    tenant_id,
                    webservice: Webservice::Wsfev1,
                };
                map.insert(
                    key,
                    TokenArca::from_cache(cached.cuit, cached.token, cached.sign, cached.expir),
                );
                debug!("Token de wsfev1 recuperado del cache en disco");
            }
        }
    }

    map
}

fn persist_token_cache(token_map: &dashmap::DashMap<ServiceId, TokenArca>, tenant_id: i64) {
    let key = ServiceId {
        tenant_id,
        webservice: Webservice::Wsfev1,
    };
    if let Some(entry) = token_map.get(&key) {
        let cached = CachedToken {
            cuit: entry.cuit(),
            token: entry.token().to_owned(),
            sign: entry.sign().to_owned(),
            expir: entry.expir(),
        };
        if let Ok(json) = serde_json::to_string(&cached) {
            let _ = fs::write(cache_path(), json);
        }
    }
}

#[tokio::test]
async fn test_fecomp_ultimo_autorizado() {
    init_tracing();

    let tenant_id = 1;
    let es_prod = false;
    let req_cli = reqwest::Client::new();
    let token_map = test_token_map_with_cache(tenant_id);
    let cert_key_getter = || test_cert_key_getter(es_prod);

    let pto_vta = 1;
    let cbte_tipo = 11; // Factura C

    let res = fecomp_ultimo_autorizado(
        token_map.clone(),
        tenant_id,
        es_prod,
        &req_cli,
        pto_vta,
        cbte_tipo,
        cert_key_getter,
    )
    .await;

    persist_token_cache(&token_map, tenant_id);

    let retorno = res.unwrap();
    debug!(?retorno);

    // println! a proposito, no debug!: el motivo de correr esto a mano es leer el
    // numero, y no siempre vas a tener el filtro de tracing en un nivel que lo muestre.
    println!(
        "Ultimo autorizado: PtoVta={} CbteTipo={} CbteNro={} -> proximo numero: {}",
        retorno.pto_vta, retorno.cbte_tipo, retorno.cbte_nro, retorno.cbte_nro + 1
    );
}