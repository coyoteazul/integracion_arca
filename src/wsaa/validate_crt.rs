use chrono::{DateTime, NaiveDateTime, Utc};
use openssl::nid::Nid;
use openssl::pkey::PKey;
use openssl::{pkey::Private, x509::X509};
use tracing::{debug, info, warn};

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
            warn!(cuit, "El certificado provisto no es un PEM valido");
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
    let private_key: Option<PKey<Private>> =
        match PKey::private_key_from_pem(private_key_pem.as_bytes()) {
            Ok(k) => Some(k),
            Err(_) => {
                warn!(cuit, "La llave privada provista no es un PEM valido");
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
                    warn!(cuit, "La llave privada no coincide con el certificado");
                    errors.push(CertError::KeyMismatch);
                }
            }
            Err(_) => {
                warn!(
                    cuit,
                    "No se pudo leer la clave publica del certificado para comparar"
                );
                errors.push(CertError::KeyMismatch)
            }
        }
    }

    // -------------------------
    // Subject: serialNumber (CUIT)
    // -------------------------
    let subject = cert.subject_name();

    let serials: Vec<String> = subject
        .entries_by_nid(Nid::SERIALNUMBER)
        .filter_map(|e| e.data().to_string().ok().map(|s| s.to_string()))
        .collect();

    let subject_serial = serials.get(0).cloned();

    if serials.is_empty() {
        warn!(cuit, "No se encontro serialNumber (CUIT) en el certificado");
        errors.push(CertError::MissingSerialNumber);
    } else {
        let val = &serials[0];

        if !val.contains(cuit.to_string().as_str()) {
            warn!(cuit, subject_serial = %val, "El CUIT del certificado no coincide con el esperado");
            errors.push(CertError::InvalidIdentidad);
        }
    }

    // -------------------------
    // Issuer: CN (must be exactly 1)
    // -------------------------
    let issuer = cert.issuer_name();

    let cns: Vec<String> = issuer
        .entries_by_nid(Nid::COMMONNAME)
        .filter_map(|e| e.data().to_string().ok().map(|s| s.to_string()))
        .collect();

    let issuer_cn = cns.get(0).cloned();

    match cns.len() {
        0 => {
            warn!(cuit, "No se encontro CN en el emisor del certificado");
            errors.push(CertError::MissingCN)
        }
        1 => {
            let expected = if es_prod {
                "Computadores"
            } else {
                "Computadores Test"
            };

            if cns[0].trim() != expected {
                warn!(cuit, issuer_cn = %cns[0], expected, "El emisor del certificado no es el esperado");
                errors.push(CertError::InvalidCN);
            }
        }
        _ => {
            warn!(
                cuit,
                cantidad = cns.len(),
                "El certificado posee mas de un CN de emisor"
            );
            errors.push(CertError::MultipleCN)
        }
    }

    // -------------------------
    // Expiration date
    // -------------------------
    let cert_venci = {
        let not_after = cert.not_after().to_string();

        match NaiveDateTime::parse_from_str(&not_after, "%b %e %H:%M:%S %Y GMT") {
            Ok(naive) => Some(DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc)),
            Err(_) => {
                warn!(cuit, not_after = %not_after, "No se pudo parsear la fecha de vencimiento del certificado");
                errors.push(CertError::InvalidDate);
                None
            }
        }
    };

    if errors.is_empty() {
        info!(cuit, cert_venci = ?cert_venci, "Certificado validado correctamente");
    } else {
        debug!(
            cuit,
            cantidad_errores = errors.len(),
            "Certificado invalido, se encontraron errores"
        );
    }

    CertInfo {
        cert_venci,
        issuer_cn,
        subject_serial,
        errors,
    }
}
