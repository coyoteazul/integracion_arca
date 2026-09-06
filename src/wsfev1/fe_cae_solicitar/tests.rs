use std::{fs, path::PathBuf, sync::Arc};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{
    tests::{init_tracing, test_cert_key_getter},
    types::{
        enums::Webservice,
        tipo_rg1415::TipoRG1415::{self, FcC011},
    },
    wsaa::get_token::{ServiceId, TokenArca},
    wsfev1::fe_cae_solicitar::{
        fecae_solicitar::fecae_solicitar,
        types::{
            ComprobCabezal, ComprobCliente, ComprobValores, Comprobante, TipoVenta::Servicios,
        },
    },
};

// --- cache de token en disco, solo para tests ---
//
// ARCA bloquea renovaciones de token demasiado frecuentes (ns1:coe.alreadyAuthenticated).
// El cache en memoria de produccion (`token_map`) no sirve entre corridas de test porque
// cada `cargo test` es un proceso nuevo. Guardamos el ultimo token vigente en un archivo
// (por defecto en el directorio temporal del SO; configurable con ARCA_TEST_TOKEN_CACHE)
// y lo reutilizamos mientras no haya vencido.

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
            // Margen de 20 minutos para no arrancar un test con un token a punto de vencer.
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
        match serde_json::to_string(&cached) {
            Ok(json) => {
                if let Err(e) = fs::write(cache_path(), json) {
                    debug!(error = ?e, "No se pudo guardar el cache de token de wsfev1");
                }
            }
            Err(e) => debug!(error = ?e, "No se pudo serializar el cache de token de wsfev1"),
        }
    }
}

// --- tests ---
//
// NOTA: punto_venta / CUIT del cliente / montos son de tu contribuyente de homologacion.
// Reemplaza los valores marcados TODO por los tuyos antes de correr esto.

#[tokio::test]
async fn test_fecae_solicitar_aprobado() {
    init_tracing();

    let tenant_id = 1;
    let es_prod = false;
    let req_cli = reqwest::Client::new();
    let token_map = test_token_map_with_cache(tenant_id);
    let cert_key_getter = || test_cert_key_getter(es_prod);

    let comprobante = Comprobante {
        id_factura: 1,
        cabezal: ComprobCabezal {
            punto_venta: 1,
            num_documento: 1,
            tipo_rg1415: TipoRG1415::FcC011,
            concepto: Servicios,
            fecha_emision: chrono::Utc::now().date_naive(),
            moneda: "PES".to_owned(),
            cotizacion: 1.0,
            cancela_misma_moneda: false,
            servicio_desde: None,
            servicio_hasta: None,
            venci_pago: None,
        },
        cliente: ComprobCliente {
            tipo_doc: 99, // Consumidor Final
            documento: 0,
            cond_iva: 5, // Consumidor Final
        },
        valores: ComprobValores {
            val_total: 100.0,
            val_nogravado: 0.0,
            val_gravado: 100.0,
            val_exento: 0.0,
            val_iva: 0.0,
            val_otros_trib: 0.0,
            tributos: None,
            alicuotas_iva: None,
        },
        comprob_asociados: None,
        periodo_asociado: None,
        opcionales: None,
        actividades: None,
    };

    let res = fecae_solicitar(
        token_map.clone(),
        tenant_id,
        es_prod,
        &req_cli,
        &comprobante,
        cert_key_getter,
    )
    .await;

    persist_token_cache(&token_map, tenant_id);

    let retorno = res.unwrap();
    debug!(?retorno);

    match retorno.resultado {
        Ok(ok) => {
            assert!(!ok.cae.is_empty());
        }
        Err(rechazo) => panic!("Se esperaba aprobacion, ARCA rechazo: {:?}", rechazo.obs),
    }
}

/// CanMisMonExt: para moneda PES, el tag no debe enviarse (cod. 10241). Este test lo
/// verifica indirectamente: si el fix se rompiera (volviera a mandarse siempre), este
/// comprobante en PES seria rechazado con 10241 en vez de aprobado.
#[tokio::test]
async fn test_fecae_solicitar_pes_no_envia_can_mis_mon_ext() {
    init_tracing();

    let tenant_id = 1;
    let es_prod = false;
    let req_cli = reqwest::Client::new();
    let token_map = test_token_map_with_cache(tenant_id);
    let cert_key_getter = || test_cert_key_getter(es_prod);

    let comprobante = Comprobante {
        id_factura: 2,
        cabezal: ComprobCabezal {
            punto_venta: 1,
            num_documento: 2,
            tipo_rg1415: FcC011,
            concepto: Servicios,
            fecha_emision: chrono::Utc::now().date_naive(),
            moneda: "PES".to_owned(),
            cotizacion: 1.0,
            cancela_misma_moneda: true, // a proposito: si el fix falla, esto dispararia 10241
            servicio_desde: None,
            servicio_hasta: None,
            venci_pago: None,
        },
        cliente: ComprobCliente {
            tipo_doc: 99,
            documento: 0,
            cond_iva: 5,
        },
        valores: ComprobValores {
            val_total: 50.0,
            val_nogravado: 0.0,
            val_gravado: 50.0,
            val_exento: 0.0,
            val_iva: 0.0,
            val_otros_trib: 0.0,
            tributos: None,
            alicuotas_iva: None,
        },
        comprob_asociados: None,
        periodo_asociado: None,
        opcionales: None,
        actividades: None,
    };

    let res = fecae_solicitar(
        token_map.clone(),
        tenant_id,
        es_prod,
        &req_cli,
        &comprobante,
        cert_key_getter,
    )
    .await;

    persist_token_cache(&token_map, tenant_id);

    let retorno = res.unwrap();
    debug!(?retorno);

    match retorno.resultado {
        Ok(ok) => assert!(!ok.cae.is_empty()),
        Err(rechazo) => {
            let tiene_10241 = rechazo.obs.iter().any(|o| o.code == "10241");
            panic!(
                "Rechazado (10241 presente: {tiene_10241}): {:?}",
                rechazo.obs
            );
        }
    }
}
