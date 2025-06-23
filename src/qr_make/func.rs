use std::path::{PathBuf};

use base64::{engine::general_purpose, Engine};
use chrono::NaiveDate;
use qrcode_generator::QrCodeEcc;
use serde::Serialize;

const QR_ARCA_URL:&str = "https://www.arca.gob.ar/fe/qr/?p=";

pub fn qr_make_file(json:&FacJson, to_file_path:PathBuf){
	let mut path = PathBuf::from(to_file_path);
	let json = serde_json::to_string(json).unwrap();
	let base64 = general_purpose::STANDARD.encode(json);
	let qr_str = dbg!(format!("{QR_ARCA_URL}{base64}"));
	let size = 200;
	path.set_extension("svg");
	dbg!(qrcode_generator::to_svg_to_file(&qr_str,
		QrCodeEcc::Low,
		size,
		None::<&str>,
		&path).unwrap());

	/*path.set_extension("png");
	qrcode_generator::to_png_to_file(&qr_str,
		QrCodeEcc::Low,
		size,
		&path).unwrap();*/
}

pub fn qr_make_base64(json:&FacJson) -> String {
	let json = serde_json::to_string(json).unwrap();
	let base64 = general_purpose::STANDARD.encode(json);
	let qr_str = dbg!(format!("{QR_ARCA_URL}{base64}"));
	//400 funciona bien para el pos de mercadopago. Deja suficiente papel de sobra despues del QR, sin exagerar
	let size = 400;

	let bytes = qrcode_generator::to_png_to_vec(qr_str, QrCodeEcc::Low, size).unwrap();
	dbg!(general_purpose::STANDARD.encode(bytes))
}



#[allow(non_snake_case)]
#[derive( Serialize, Debug)]
pub struct FacJson {
	pub ver				 :i32,
	pub fecha			 :NaiveDate,
	pub cuit			 :i64,
	pub ptoVta		 :i64,
	pub tipoCmp		 :i64,
	pub nroCmp		 :i64,
	pub importe		 :f64,
	pub moneda		 :String,
	pub ctz				 :f64,
	pub tipoDocRec :i64,
	pub nroDocRec	 :i64,
	pub tipoCodAut :String,
	pub codAut		 :i64
}