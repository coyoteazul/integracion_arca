#[cfg(test)]
mod tests {
	use std::path::PathBuf;

use chrono::Utc;

  use crate::qr_make::func::{qr_make_base64, qr_make_file, FacJson};

	#[test]
	fn qr_a_archivo() {
		let json = FacJson{ 
			ver: 1, 
			fecha: Utc::now().date_naive(),
			cuit: 20398305923, 
			ptoVta: 1, 
			tipoCmp: 11, 
			nroCmp: 200, 
			importe: 20000.0, 
			moneda: "PES".to_owned(), 
			ctz: 1.0, 
			tipoDocRec: 99, 
			nroDocRec: 0, 
			tipoCodAut: "E".to_owned(),
			codAut: 1234
		};

		let to_file_path = PathBuf::from("./qr.png");

		qr_make_file(&json, to_file_path);
	}

		#[test]
	fn qr_a_base64() {
		let json = FacJson{ 
			ver: 1, 
			fecha: Utc::now().date_naive(),
			cuit: 20398305923, 
			ptoVta: 1, 
			tipoCmp: 11, 
			nroCmp: 200, 
			importe: 20000.0, 
			moneda: "PES".to_owned(), 
			ctz: 1.0, 
			tipoDocRec: 99, 
			nroDocRec: 0, 
			tipoCodAut: "E".to_owned(),
			codAut: 1234
		};

		

		qr_make_base64(&json);
	}

	

}