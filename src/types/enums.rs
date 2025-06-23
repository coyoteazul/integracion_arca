use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Webservice {
	Wsaa,
	Wsfev1
}


impl fmt::Display for Webservice {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			let s = match self {
					Webservice::Wsaa => "wsaa",
					Webservice::Wsfev1 => "wsfe",
			};
			write!(f, "{}", s)
	}
}