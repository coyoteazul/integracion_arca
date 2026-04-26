use openssl::pkey::PKey;
use openssl::{pkey::Private, x509::X509};
use openssl::nid::Nid;
use chrono::{DateTime, Utc, NaiveDateTime};


#[derive(Debug)]
pub struct CertInfo {
    pub cert_venci: Option<DateTime<Utc>>,
    pub issuer_cn: Option<String>,
    pub subject_serial: Option<String>,
    pub errors: Vec<CertError>,
}

#[derive(Debug)]
pub enum CertError {
    InvalidPem,
    MissingSerialNumber,
    InvalidIdentidad,
    MissingCN,
    MultipleCN,
    InvalidCN,
    InvalidDate,
		InvalidPrivateKey,
    KeyMismatch,
}

impl CertError {
	pub fn to_string(&self) -> String {
		match self {
				CertError::InvalidPem          => "Certificado Invalido".to_string(),
				CertError::MissingSerialNumber => "No se encontro el CUIT".to_string(),
				CertError::InvalidIdentidad    => "El certificado no pertenece a esta empresa".to_string(),
				CertError::MissingCN           => "No se pudo identificar al emisor del certificado".to_string(),
				CertError::MultipleCN          => "El certificado posee mas de un emisor".to_string(),
				CertError::InvalidCN           => "El emisor del certificado no es el esperado. Se espera 'Computadores' para produccion y 'Computadores Test' para homologacion".to_string(),
				CertError::InvalidDate         => "No se pudo leer la fecha de vencimiento del certificado".to_string(),
				CertError::InvalidPrivateKey   => "No se pudo leer la llave privada del certificado".to_string(),
				CertError::KeyMismatch         => "La llave privada no coincide con el certificado".to_string(),
		}
	}
}

pub fn inspect_cert(cert_pem: &str, es_prod: bool, cuit: i64, private_key_pem: &str) -> CertInfo {
    let mut errors = Vec::new();

    // -------------------------
    // Parse certificate
    // -------------------------
    let cert = match X509::from_pem(cert_pem.as_bytes()) {
        Ok(c) => c,
        Err(_) => {
            return CertInfo {
                cert_venci: None,
                issuer_cn: None,
                subject_serial: None,
                errors: vec![CertError::InvalidPem],
            };
        }
    };

		// -------------------------
    // Parse private key
    // -------------------------
    let private_key: Option<PKey<Private>> = match PKey::private_key_from_pem(private_key_pem.as_bytes()) {
        Ok(k) => Some(k),
        Err(_) => {
            errors.push(CertError::InvalidPrivateKey);
            None
        }
    };

    // -------------------------
    // Validate key matches cert
    // -------------------------
    if let Some(pk) = &private_key {
        match cert.public_key() {
            Ok(cert_pub) => {
                if !pk.public_eq(&cert_pub) {
                    errors.push(CertError::KeyMismatch);
                }
            }
            Err(_) => errors.push(CertError::KeyMismatch),
        }
    }


    // -------------------------
    // Subject: serialNumber (CUIT)
    // -------------------------
    let subject = cert.subject_name();

    let serials: Vec<String> = subject
        .entries_by_nid(Nid::SERIALNUMBER)
        .filter_map(|e| e.data().as_utf8().ok().map(|s| s.to_string()))
        .collect();

    let subject_serial = serials.get(0).cloned();

    if serials.is_empty() {
        errors.push(CertError::MissingSerialNumber);
    } else {
        let val = &serials[0];

        if !val.contains(cuit.to_string().as_str()) {
            errors.push(CertError::InvalidIdentidad);
        }
    }

    // -------------------------
    // Issuer: CN (must be exactly 1)
    // -------------------------
    let issuer = cert.issuer_name();

    let cns: Vec<String> = issuer
        .entries_by_nid(Nid::COMMONNAME)
        .filter_map(|e| e.data().as_utf8().ok().map(|s| s.to_string()))
        .collect();

    let issuer_cn = cns.get(0).cloned();

    match cns.len() {
        0 => errors.push(CertError::MissingCN),
        1 => {
            let expected = if es_prod {
                "Computadores"
            } else {
                "Computadores Test"
            };

            if cns[0].trim() != expected {
                errors.push(CertError::InvalidCN);
            }
        }
        _ => errors.push(CertError::MultipleCN),
    }

    // -------------------------
    // Expiration date
    // -------------------------
    let cert_venci = {
        let not_after = cert.not_after().to_string();

        match NaiveDateTime::parse_from_str(
            &not_after,
            "%b %e %H:%M:%S %Y GMT"
        ) {
            Ok(naive) => Some(DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc)),
            Err(_) => {
                errors.push(CertError::InvalidDate);
                None
            }
        }
    };

    CertInfo {
        cert_venci,
        issuer_cn,
        subject_serial,
        errors,
    }
}