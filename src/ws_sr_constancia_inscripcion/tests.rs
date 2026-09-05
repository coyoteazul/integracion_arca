use tracing::debug;

use crate::{
    tests::{init_tracing, test_cert_key_getter, test_token_map},
    ws_sr_constancia_inscripcion::{get_by_cuit::get_by_cuit, types::ConstanciaInscripcion},
};

async fn dispara_test(cuit: i64) -> ConstanciaInscripcion {
    init_tracing();

    let token_map = test_token_map();
    let tenant_id = 1;
    let es_prod = true;
    let req_cli = reqwest::Client::new();
    let cert_key_getter = || test_cert_key_getter(es_prod);

    let res = get_by_cuit(
        token_map,
        tenant_id,
        es_prod,
        &req_cli,
        cuit,
        cert_key_getter,
    )
    .await
    .unwrap();

    debug!(?res);
    res.parsed
}

#[tokio::test]
async fn test_get_by_cuit() {
    let res1 = dispara_test(30696155190).await;
    let res2 = dispara_test(20218346414).await;
    let res3 = dispara_test(30547981029).await;
    let res4 = dispara_test(20224323116).await;
    let res5 = dispara_test(27113929117).await;

    assert!(res1.nombre.razon_social.unwrap().contains("TOTVS"));

    assert!(res2.nombre.apellido.unwrap().contains("MILEI"));

    assert!(res3.nombre.razon_social.unwrap().contains("TECHIN"));

    assert!(res4.nombre.apellido.unwrap().contains("GALPERIN"));

    assert!(res5.nombre.apellido.unwrap().contains("KIRCHNER"));

    assert!(
        res5.errores
            .iter()
            .any(|x| x.contains("La CUIT registra pendiente"))
    );
}
