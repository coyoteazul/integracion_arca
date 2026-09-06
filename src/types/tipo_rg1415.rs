use std::fmt;

use serde::{Deserialize, Serialize};

use crate::types::tipo_rg1415::{Especie::*, TipoRG1415::*};

#[repr(i32)]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum TipoRG1415 {
    //A
    FcA001 = 001,
    NdA002 = 002,
    NcA003 = 003,
    //B
    FcB006 = 006,
    NdB007 = 007,
    NcB008 = 008,
    //C
    FcC011 = 011,
    NdC012 = 012,
    NcC013 = 013,
    //A con leyenda
    FcA051 = 051,
    NdA052 = 052,
    NcA053 = 053,
    //A Pyme
    FcA201 = 201,
    NdA202 = 202,
    NcA203 = 203,
    //B Pyme
    FcB206 = 206,
    NdB207 = 207,
    NcB208 = 208,
    //C Pyme
    FcC211 = 211,
    NdC212 = 212,
    NcC213 = 213,
    //invalid
    Invalid = 9999,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum Especie {
    Factura,
    NDebito,
    NCredit,
    Invalid,
}

impl fmt::Display for TipoRG1415 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl TipoRG1415 {
    pub fn get_anul(&self) -> TipoRG1415 {
        match self {
            TipoRG1415::FcA001 => TipoRG1415::NcA003,
            TipoRG1415::NdA002 => TipoRG1415::NcA003,
            TipoRG1415::NcA003 => TipoRG1415::NdA002,
            TipoRG1415::FcB006 => TipoRG1415::NcB008,
            TipoRG1415::NdB007 => TipoRG1415::NcB008,
            TipoRG1415::NcB008 => TipoRG1415::NdB007,
            TipoRG1415::FcC011 => TipoRG1415::NcC013,
            TipoRG1415::NdC012 => TipoRG1415::NcC013,
            TipoRG1415::NcC013 => TipoRG1415::NdC012,
            TipoRG1415::FcA051 => TipoRG1415::NcA053,
            TipoRG1415::NdA052 => TipoRG1415::NcA053,
            TipoRG1415::NcA053 => TipoRG1415::NdA052,
            TipoRG1415::FcA201 => TipoRG1415::NcA203,
            TipoRG1415::NdA202 => TipoRG1415::NcA203,
            TipoRG1415::NcA203 => TipoRG1415::NdA202,
            TipoRG1415::FcB206 => TipoRG1415::NcB208,
            TipoRG1415::NdB207 => TipoRG1415::NcB208,
            TipoRG1415::NcB208 => TipoRG1415::NdB207,
            TipoRG1415::FcC211 => TipoRG1415::NcC213,
            TipoRG1415::NdC212 => TipoRG1415::NcC213,
            TipoRG1415::NcC213 => TipoRG1415::NdC212,
            _ => TipoRG1415::Invalid,
        }
    }

    pub fn get_info(&self) -> &'static ComprobTipo {
        COMPROB_TIPO_INFO
            .iter()
            .find(|info| info.tipo == *self)
            .unwrap_or(&COMPROB_TIPO_INFO[0])
    }
}

impl From<i32> for TipoRG1415 {
		fn from(value: i32) -> Self {
				COMPROB_TIPO_INFO
						.iter()
						.find(|info| info.codigo == value)
						.map(|info| info.tipo)
						.unwrap_or(TipoRG1415::Invalid)
		}
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ComprobTipo {
    pub tipo: TipoRG1415,
    pub especie: Especie,
    pub letra: char,
    pub es_pyme: bool,
    pub con_leyenda: bool,
		pub codigo: i32,
}

pub const COMPROB_TIPO_INFO: [ComprobTipo; 22] = [
		ComprobTipo { tipo: TipoRG1415::Invalid, codigo:9999, especie: Especie::Invalid, letra: 'X', es_pyme: false, con_leyenda: false },

		ComprobTipo { tipo: FcA001, codigo: 001, especie: Factura, letra: 'A', es_pyme: false, con_leyenda: false, },
		ComprobTipo { tipo: NdA002, codigo: 002, especie: NDebito, letra: 'A', es_pyme: false, con_leyenda: false, },
		ComprobTipo { tipo: NcA003, codigo: 003, especie: NCredit, letra: 'A', es_pyme: false, con_leyenda: false, },
		ComprobTipo { tipo: FcB006, codigo: 006, especie: Factura, letra: 'B', es_pyme: false, con_leyenda: false, },
		ComprobTipo { tipo: NdB007, codigo: 007, especie: NDebito, letra: 'B', es_pyme: false, con_leyenda: false, },
		ComprobTipo { tipo: NcB008, codigo: 008, especie: NCredit, letra: 'B', es_pyme: false, con_leyenda: false, },
		ComprobTipo { tipo: FcC011, codigo: 011, especie: Factura, letra: 'C', es_pyme: false, con_leyenda: false, },
		ComprobTipo { tipo: NdC012, codigo: 012, especie: NDebito, letra: 'C', es_pyme: false, con_leyenda: false, },
		ComprobTipo { tipo: NcC013, codigo: 013, especie: NCredit, letra: 'C', es_pyme: false, con_leyenda: false, },
		ComprobTipo { tipo: FcA051, codigo: 051, especie: Factura, letra: 'A', es_pyme: false, con_leyenda: true,  },
		ComprobTipo { tipo: NdA052, codigo: 052, especie: NDebito, letra: 'A', es_pyme: false, con_leyenda: true,  },
		ComprobTipo { tipo: NcA053, codigo: 053, especie: NCredit, letra: 'A', es_pyme: false, con_leyenda: true,  },
		ComprobTipo { tipo: FcA201, codigo: 201, especie: Factura, letra: 'A', es_pyme: true, con_leyenda: false,  },
		ComprobTipo { tipo: NdA202, codigo: 202, especie: NDebito, letra: 'A', es_pyme: true, con_leyenda: false,  },
		ComprobTipo { tipo: NcA203, codigo: 203, especie: NCredit, letra: 'A', es_pyme: true, con_leyenda: false,  },
		ComprobTipo { tipo: FcB206, codigo: 206, especie: Factura, letra: 'B', es_pyme: true, con_leyenda: false,  },
		ComprobTipo { tipo: NdB207, codigo: 207, especie: NDebito, letra: 'B', es_pyme: true, con_leyenda: false,  },
		ComprobTipo { tipo: NcB208, codigo: 208, especie: NCredit, letra: 'B', es_pyme: true, con_leyenda: false,  },
		ComprobTipo { tipo: FcC211, codigo: 211, especie: Factura, letra: 'C', es_pyme: true, con_leyenda: false,  },
		ComprobTipo { tipo: NdC212, codigo: 212, especie: NDebito, letra: 'C', es_pyme: true, con_leyenda: false,  },
		ComprobTipo { tipo: NcC213, codigo: 213, especie: NCredit, letra: 'C', es_pyme: true, con_leyenda: false,  },		
];
