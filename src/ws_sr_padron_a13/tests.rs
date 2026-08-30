use crate::{tests::{test_cert_key_getter, test_token_map}, ws_sr_padron_a13::{get_by_cuit::get_by_cuit, get_by_dni::get_by_dni}};

#[tokio::test]
async fn test_get_by_dni() {
	
  let token_map = test_token_map();
  let tenant_id = 1;
  let es_prod = true;
  let req_cli = reqwest::Client::new();
  let dni = 36602558;
  let cert_key_getter = test_cert_key_getter;

	let res = get_by_dni(token_map, tenant_id, es_prod, &req_cli, dni, cert_key_getter).await.unwrap();

	dbg!(&res);
	assert!(res.len() == 2);

	assert!(res.first().unwrap().parsed.nombre.apellido.clone().unwrap().to_uppercase().contains("RUIZ"))

}


#[tokio::test]
async fn test_get_by_cuit() {
	
  let token_map = test_token_map();
  let tenant_id = 1;
  let es_prod = true;
  let req_cli = reqwest::Client::new();
  let cuit = 30696155190;
  let cert_key_getter = test_cert_key_getter;

	let res = get_by_cuit(token_map, tenant_id, es_prod, &req_cli, cuit, cert_key_getter).await.unwrap();

	dbg!(&res);
	assert!(res.parsed.nombre.razon_social.clone().unwrap().to_uppercase().contains("TOTVS"))

}