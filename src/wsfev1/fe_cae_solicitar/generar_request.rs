use std::{sync::Arc, time::Duration};

use chrono::NaiveDate;
use reqwest::{header::CONTENT_TYPE, Client, RequestBuilder};

use crate::{types::{enums::Webservice, errors::ErrType}, wsaa::get_token::{get_token, CertKeyPair, ServiceId, TokenArca}, wsfev1::url::{WSFEV1_URL_HOMO, WSFEV1_URL_PROD}};

/// Genera el request completamente, incluyendo auth y contenido, pero no lo envia.
/// De esta forma podes logear el contenido antes de enviarlo
/// `cert_key_getter` Solo se llama si es necesario renovar el token
pub async fn generar_request<Fc>(
	token_map				: Arc<dashmap::DashMap<ServiceId, TokenArca>>,
	tenant_id				: i64,
	es_prod					: bool,
	req_cli					: &Client,
	comprobante		 	: &Comprobante,
	cert_key_getter	: Fc,
) -> Result<(RequestBuilder, String), ErrType>
where 
	Fc: AsyncFnMut() -> Option<CertKeyPair>,
{
	let url = if es_prod {WSFEV1_URL_PROD} else {WSFEV1_URL_HOMO};
	let key = ServiceId{ tenant_id, webservice: Webservice::Wsfev1 };
	let auth_xml = get_token(token_map, key, es_prod, req_cli, cert_key_getter, token_parser).await?;

	let send_xml = xml_make(comprobante, auth_xml);

	let req = req_cli.post(url)
	.header(CONTENT_TYPE, "application/soap+xml")
	.body(send_xml.clone())
	.timeout(Duration::from_secs(60));

	return Ok((req, send_xml));
}

fn token_parser(cuit:i64, token:&str, sign:&str ) -> String {
	format!(
r#"<ar:Auth>
	<ar:Token>{token}</ar:Token>
	<ar:Sign>{sign}</ar:Sign>
	<ar:Cuit>{cuit}</ar:Cuit>
</ar:Auth>"#)
}


#[derive(Debug)]
pub struct Comprobante {
	pub id_factura:i64,
	pub cabezal: ComprobCabezal,
	pub cliente: ComprobCliente,
	pub valores: ComprobValores,

	pub comprob_asociados	: Option<Vec<ComprobAsoc>>,
	pub periodo_asociado	: Option<ComprobPeriodo>,
	pub opcionales				: Option<Vec<ComprobOpcionales>>,
	pub actividades				: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct ComprobCabezal {
	pub punto_venta 				: i64,
	pub num_documento				: i64,
	pub tipo_rg1415 				: i64,
	///1:Productos, 2:Servicios, 3:Ambos
	pub concepto						: i8,
	pub fecha_emision 			: NaiveDate,
	pub moneda							: String,
	pub cotizacion					: f64,
	pub cancela_misma_moneda: bool,
	pub servicio_desde			: Option<NaiveDate>,
	pub servicio_hasta			: Option<NaiveDate>,
	pub venci_pago		 			: Option<NaiveDate>,
}

#[derive(Debug)]
pub struct ComprobCliente {
	pub tipo_doc 	: i64,
	pub documento	: i64,
	pub cond_iva 	: i64,
}

#[derive(Debug)]
pub struct ComprobValores {
	pub val_total			: f64,
	pub val_nogravado	: f64,
	pub val_gravado		: f64,
	pub val_exento	 	: f64,
	pub val_iva				: f64,
	pub val_otros_trib: f64,
	pub tributos			: Option<Vec<ComprobTributos>>,
	pub alicuotas_iva	: Option<Vec<ComprobIVA>>,
}

#[derive(Debug)]
pub struct ComprobAsoc {
	pub punto_venta 	: i64,
	pub num_documento	: i64,
	pub tipo_rg1415 	: i64,
	pub fecha_emision : NaiveDate,
}

#[derive(Debug)]
pub struct ComprobTributos {
	pub id_tributo: i64,
	pub desc 			: String,
	pub base 			: f64,
	pub alicuota 	: f64,
	pub importe 	: f64
}

#[derive(Debug)]
pub struct ComprobIVA {
	pub id_alicuota : i64,
	pub base 				: f64,
	pub importe		 	: f64,
}

#[derive(Debug)]
pub struct ComprobPeriodo {
	pub fecha_desde : NaiveDate,
	pub fecha_hasta : NaiveDate,
}

#[derive(Debug)]
pub struct ComprobOpcionales {
	pub id 		: String,
	pub valor : String,
}



fn xml_make(comp: &Comprobante, auth_xml:String) -> String {
	const COMP_TIPO_C:[i64;3] = [11,12,13];
	let ComprobCabezal{ punto_venta, num_documento, tipo_rg1415, concepto, fecha_emision, moneda, cotizacion, cancela_misma_moneda, servicio_desde, servicio_hasta, venci_pago} = &comp.cabezal;
	let ComprobCliente{ tipo_doc, documento, cond_iva } = comp.cliente;
	let &ComprobValores{ ref val_total, mut val_nogravado, mut val_gravado, ref val_exento, ref val_iva, ref val_otros_trib, ref tributos, ref alicuotas_iva } = &comp.valores;
	let fecha_emision = fecha_emision.format("%Y%m%d").to_string();
	let cancela_misma_moneda = if *cancela_misma_moneda {'S'} else {'N'};

	if COMP_TIPO_C.contains(tipo_rg1415) {
		val_gravado = val_nogravado.clone();
		val_nogravado = 0.0;
	}

	let fe_cab_ret = format!(
r#"<ar:FeCabReq>
	<ar:CantReg>1</ar:CantReg>
	<ar:PtoVta>{punto_venta}</ar:PtoVta>
	<ar:CbteTipo>{tipo_rg1415}</ar:CbteTipo>
</ar:FeCabReq>"#);

	let fecha_serv = match (servicio_desde,servicio_hasta) {
		(Some(desde), Some(hasta)) => {
			let desde = desde.format("%Y%m%d").to_string();
			let hasta = hasta.format("%Y%m%d").to_string();
			format!(
r#"<ar:FchServDesde>{desde}</ar:FchServDesde>
<ar:FchServHasta>{hasta}</ar:FchServHasta>"#)
		},
		_ => String::new()
	};

	let periodo_asoc = if let Some(periodo) = &comp.periodo_asociado {
		let desde = periodo.fecha_desde.format("%Y%m%d").to_string();
			let hasta = periodo.fecha_hasta.format("%Y%m%d").to_string();
			format!(
r#"<ar:PeriodoAsoc>
		<ar:FchDesde>{desde}</ar:FchDesde>
		<ar:FchHasta>{hasta}</ar:FchHasta>
</ar:PeriodoAsoc>"#)
	} else {String::new()};
	

	let fecha_venc = if let Some(venci) = venci_pago {
		let venci = venci.format("%Y%m%d").to_string();
		format!(r#"<ar:FchVtoPago>{venci}</ar:FchVtoPago>"#)
	} else {String::new()};

	let comp_asoc = cbte_asoc_xml(&comp.comprob_asociados);
	let tribut = tributos_xml(&tributos);
	let iva = ivaalic_xml(alicuotas_iva);
	let opcion = opcion_xml(&comp.opcionales);
	let activid = actividades_xml(&comp.actividades);

	let det_request = format!(
r#"<ar:FeDetReq>
	<ar:FECAEDetRequest>
		<ar:Concepto>{concepto}</ar:Concepto>
		<ar:DocTipo>{tipo_doc}</ar:DocTipo>
		<ar:DocNro>{documento}</ar:DocNro>
		<ar:CbteDesde>{num_documento}</ar:CbteDesde>
		<ar:CbteHasta>{num_documento}</ar:CbteHasta>
		<ar:CbteFch>{fecha_emision}</ar:CbteFch>
		<ar:ImpTotal>{val_total}</ar:ImpTotal>
		<ar:ImpTotConc>{val_nogravado}</ar:ImpTotConc>
		<ar:ImpNeto>{val_gravado}</ar:ImpNeto>
		<ar:ImpOpEx>{val_exento}</ar:ImpOpEx>
		<ar:ImpTrib>{val_otros_trib}</ar:ImpTrib>
		<ar:ImpIVA>{val_iva}</ar:ImpIVA>
		<ar:MonId>{moneda}</ar:MonId>
		<ar:MonCotiz>{cotizacion}</ar:MonCotiz>
		<ar:CanMisMonExt>{cancela_misma_moneda}</ar:CanMisMonExt>
		<ar:CondicionIVAReceptorId>{cond_iva}</ar:CondicionIVAReceptorId>
		{fecha_serv}
		{fecha_venc}
		{comp_asoc}
		{tribut}
		{iva}
		{opcion}
		{periodo_asoc}
		{activid}
	</ar:FECAEDetRequest>
</ar:FeDetReq>"#);


format!(r#"<soap:Envelope xmlns:soap="http://www.w3.org/2003/05/soap-envelope" xmlns:ar="http://ar.gov.afip.dif.FEV1/">
   <soap:Header/>
   <soap:Body>
      <ar:FECAESolicitar>
				{auth_xml}
				<ar:FeCAEReq>
					{fe_cab_ret}
					{det_request}
				</ar:FeCAEReq>
			</ar:FECAESolicitar>
   </soap:Body>
</soap:Envelope>"#)
}


fn cbte_asoc_xml(com:&Option<Vec<ComprobAsoc>>) -> String {
	if let Some(asoc) = com {
		if asoc.len() > 0 {
			let ar = asoc.iter()
			.map(|f|{
				let ComprobAsoc { punto_venta, num_documento, tipo_rg1415, fecha_emision} = &f;
				let fecha_emision = fecha_emision.format("%Y%m%d").to_string();
				format!(
r#"
<ar:CbteAsoc>
	<ar:Tipo>{tipo_rg1415}</ar:Tipo>
	<ar:PtoVta>{punto_venta}</ar:PtoVta>
	<ar:Nro>{num_documento}</ar:Nro>
	<ar:CbteFch>{fecha_emision}</ar:CbteFch>
</ar:CbteAsoc>"#)
			})
			.reduce(|acc, val| {
				acc + &val
			}).unwrap();

			return format!(r#"<ar:CbtesAsoc>{ar}</ar:CbtesAsoc>"#)
		}	
	};

	String::new()
}


fn tributos_xml(trib:&Option<Vec<ComprobTributos>>) -> String {
	if let Some(trib) = trib {
		if trib.len() > 0 {
			let ar = trib.iter()
			.map(|f|{
				let ComprobTributos { id_tributo, desc, base, alicuota, importe } = f;
				format!(
r#"<ar:Tributo>
	<ar:Id>{id_tributo}</ar:Id>
	<ar:Desc>{desc}</ar:Desc>
	<ar:BaseImp>{base}</ar:BaseImp>
	<ar:Alic>{alicuota}</ar:Alic>
	<ar:Importe>{importe}</ar:Importe>
</ar:Tributo>"#)
			})
			.reduce(|acc, val| {
				acc + &val
			}).unwrap();

			return format!(r#"<ar:Tributos>{ar}</ar:Tributos>"#)
		}	
	};

	String::new()
}


fn ivaalic_xml(iva:&Option<Vec<ComprobIVA>>) -> String {
	if let Some(iva) = iva {
		if iva.len() > 0 {
			let ar = iva.iter()
			.map(|f|{
				let ComprobIVA { id_alicuota, base, importe } = f;
				format!(
r#"<ar:AlicIva>
	<ar:Id>{id_alicuota}</ar:Id>
	<ar:BaseImp>{base}</ar:BaseImp>
	<ar:Importe>{importe}</ar:Importe>
</ar:AlicIva>"#)
			})
			.reduce(|acc, val| {
				acc + &val
			}).unwrap();

			return format!(r#"<ar:Iva>{ar}</ar:Iva>"#)
		}	
	};

	String::new()
}


fn opcion_xml(iva:&Option<Vec<ComprobOpcionales>>) -> String {
	if let Some(iva) = iva {
		if iva.len() > 0 {
			let ar = iva.iter()
			.map(|f|{
				let ComprobOpcionales { id, valor } = f;
				format!(
r#"<ar:Opcional>
	<ar:Id>{id}</ar:Id>
	<ar:Valor>{valor}</ar:Valor>
</ar:Opcional>"#)
			})
			.reduce(|acc, val| {
				acc + &val
			}).unwrap();

			return format!(r#"<ar:Opcionales>{ar}</ar:Opcionales>"#)
		}	
	};

	String::new()
}


fn actividades_xml(activ:&Option<Vec<String>>) -> String {
	if let Some(activ) = activ {
		if activ.len() > 0 {
			let ar = activ.iter()
			.map(|f|{
				format!(
r#"<ar:Actividad>
	<ar:Id>{f}</ar:Id>
</ar:Actividad>"#)})
			.reduce(|acc, val| {acc + &val})
			.unwrap();

			return format!(r#"<ar:Actividades>{ar}</ar:Actividades>"#)
		}	
	};

	String::new()
}