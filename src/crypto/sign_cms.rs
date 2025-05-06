use openssl::cms::CMSOptions;
use openssl::pkey::PKey;
use openssl::x509::X509;

/**
 * Recibe data y la encripta y firma con el cert y key recibidos
 */
pub fn sign_cms(
	cert_contents:&Vec<u8>,
	key_contents:&Vec<u8>,
	data:&str
) -> String {
	const LEN_BEGIN:usize = "-----BEGIN CMS-----".len();
	const LEN_END:usize   = "-----END CMS-----".len()+1;

	/*let cert_contents = fs::read_to_string(cert_path).unwrap();
	let key_contents = fs::read_to_string(key_path).unwrap();*/

	let cert = X509::from_pem(cert_contents)
		.expect("No se pudo leer el certificado como X509");
	let key  = PKey::private_key_from_pem(key_contents)
		.expect("No se pudo leer la key como PKey");

	let flags = CMSOptions::empty();
	let mut pem = openssl::cms::CmsContentInfo::sign(
		Some(cert.as_ref()), 
		Some(key.as_ref()),
		None,
		Some(data.as_bytes()),
		flags
	).unwrap().to_pem().unwrap();

	//remover cabezales
  pem.drain(pem.len() - LEN_END..);
	pem.drain(0..LEN_BEGIN);

	return String::from_utf8(pem).unwrap();
}