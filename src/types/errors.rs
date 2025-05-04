use crate::xml_utils::get_xml_tag;

pub enum ErrType {
	Soap(SoapFault),
	Req(reqwest::Error),
	Serde(serde_json::Error),
	Sqlx(sqlx::Error),
}

impl From<SoapFault> for ErrType {
	fn from(err:SoapFault) -> ErrType {
		ErrType::Soap(err)
	}
}

impl From<reqwest::Error> for ErrType {
	fn from(err:reqwest::Error) -> ErrType {
		ErrType::Req(err)
	}
}

impl From<serde_json::Error> for ErrType {
	fn from(err:serde_json::Error) -> ErrType {
		ErrType::Serde(err)
	}
}

impl From<sqlx::Error> for ErrType {
	fn from(err:sqlx::Error) -> ErrType {
		ErrType::Sqlx(err)
	}
}

#[derive(Debug)]
pub struct SoapFault {
	pub fault_code  : Option<String>,
	pub fault_string: Option<String>, 
}

impl SoapFault {
	pub fn new(fault_code:&str, fault_string:&str) -> Self {
		SoapFault { fault_code: Some(fault_code.to_owned()), fault_string:Some(fault_string.to_owned()) }
	}
	pub fn from_xml(xml:&str) ->Self {
		Self{
			fault_string: get_xml_tag(xml,"faultstring"),
			fault_code: get_xml_tag(xml,"faultcode")
		}
	}
}