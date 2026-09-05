use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum TipoPersona {
    Fisica,
    Juridica,
    Desconocida,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum TipoClave {
    Cuit,
    Cuil,
    Cdi,
    Desconocida,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum EstadoClave {
    Activo,
    Inactivo,
    Desconocido,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum TipoDomicilio {
    Fiscal,
    Legalreal,
    Desconocido,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Clave {
    pub id_persona: String,
    pub tipo_clave: TipoClave,
    pub estado_clave: EstadoClave,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Documento {
    pub numero_documento: String,
    pub tipo_documento: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Provincia {
    pub id_provincia: i32,
    pub descripcion_provincia: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DatoAdicional {
    pub tipo_dato_adicional: String,
    pub dato_adicional: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Direccion {
    pub direccion: String,
    pub calle: Option<String>,
    pub numero: Option<String>,
    pub codigo_postal: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Domicilio {
    pub tipo_domicilio: TipoDomicilio,
    pub direccion: Direccion,
    pub localidad: String,
    pub provincia: Provincia,
    pub datos_adicionales: Vec<DatoAdicional>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Nombre {
    pub nombre: Option<String>,
    pub apellido: Option<String>,
    pub razon_social: Option<String>,
}

impl Nombre {
    pub fn to_string(&self) -> String {
        if let Some(razon_social) = &self.razon_social {
            razon_social.clone()
        } else if let Some(nombre) = &self.nombre {
            if let Some(apellido) = &self.apellido {
                format!("{} {}", nombre, apellido)
            } else {
                nombre.clone()
            }
        } else if let Some(apellido) = &self.apellido {
            apellido.clone()
        } else {
            String::new()
        }
    }
}

impl From<Option<&str>> for TipoPersona {
    fn from(value: Option<&str>) -> Self {
        match value {
            Some("FISICA") => Self::Fisica,
            Some("JURIDICA") => Self::Juridica,
            _ => Self::Desconocida,
        }
    }
}

impl From<Option<&str>> for TipoClave {
    fn from(value: Option<&str>) -> Self {
        match value {
            Some("CUIT") => Self::Cuit,
            Some("CUIL") => Self::Cuil,
            Some("CDI") => Self::Cdi,
            _ => Self::Desconocida,
        }
    }
}

impl From<Option<&str>> for EstadoClave {
    fn from(value: Option<&str>) -> Self {
        match value {
            Some("ACTIVO") => Self::Activo,
            Some("INACTIVO") => Self::Inactivo,
            _ => Self::Desconocido,
        }
    }
}

// a13's raw tipoDomicilio is a mandatory String; constancia's is Option<String>.
// Keep both conversions so each service can use whichever shape it actually has.
impl From<&str> for TipoDomicilio {
    fn from(value: &str) -> Self {
        match value {
            "FISCAL" => Self::Fiscal,
            "LEGAL/REAL" => Self::Legalreal,
            _ => Self::Desconocido,
        }
    }
}

impl From<Option<&str>> for TipoDomicilio {
    fn from(value: Option<&str>) -> Self {
        match value {
            Some(v) => v.into(),
            None => Self::Desconocido,
        }
    }
}
