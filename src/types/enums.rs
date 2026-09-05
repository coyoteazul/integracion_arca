use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Webservice {
    Wsaa,
    Wsfev1,
    WsSrPadronA13,
    WsSrConstanciaInscripcion,
}

/// Se usa para identificar el webservice cuando se solicita un token
impl fmt::Display for Webservice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Webservice::Wsaa => "wsaa",
            Webservice::Wsfev1 => "wsfe",
            Webservice::WsSrPadronA13 => "ws_sr_padron_a13",
            Webservice::WsSrConstanciaInscripcion => "ws_sr_constancia_inscripcion",
        };
        write!(f, "{}", s)
    }
}
