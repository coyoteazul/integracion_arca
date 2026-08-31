use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PersonaParse {
    #[serde(rename = "idPersona")]
    pub id_persona: Option<String>,

    #[serde(rename = "tipoPersona")]
    pub tipo_persona: Option<String>,

    #[serde(rename = "tipoClave")]
    pub tipo_clave: Option<String>,

    #[serde(rename = "estadoClave")]
    pub estado_clave: Option<String>,

    pub nombre: Option<String>,
    pub apellido: Option<String>,

    #[serde(rename = "razonSocial")]
    pub razon_social: Option<String>,

    #[serde(rename = "tipoDocumento")]
    pub tipo_documento: Option<String>,

    #[serde(rename = "numeroDocumento")]
    pub numero_documento: Option<String>,

    #[serde(rename = "mesCierre")]
    pub mes_cierre: Option<i32>,

    #[serde(rename = "fechaInscripcion")]
    pub fecha_inscripcion: Option<DateTime<FixedOffset>>,

    #[serde(rename = "fechaContratoSocial")]
    pub fecha_contrato_social: Option<DateTime<FixedOffset>>,

    #[serde(rename = "formaJuridica")]
    pub forma_juridica: Option<String>,

    #[serde(rename = "fechaFallecimiento")]
    pub fecha_fallecimiento: Option<DateTime<FixedOffset>>,

    #[serde(rename = "fechaNacimiento")]
    pub fecha_nacimiento: Option<DateTime<FixedOffset>>,

    #[serde(rename = "idActividadPrincipal")]
    pub id_actividad_principal: Option<i64>,

    #[serde(rename = "descripcionActividadPrincipal")]
    pub descripcion_actividad_principal: Option<String>,

    #[serde(rename = "domicilio", default)]
    pub domicilio: Vec<DomicilioParse>,

    #[serde(rename = "claveInactivaAsociada", default)]
    pub clave_inactiva_asociada: Vec<String>,

    #[serde(rename = "periodoActividadPrincipal")]
    pub periodo_actividad_principal: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct DomicilioParse {
    #[serde(rename = "tipoDomicilio")]
    pub tipo_domicilio: String,

    pub direccion: String,
    pub calle: Option<String>,
    pub numero: Option<String>,
    pub localidad: Option<String>,

    #[serde(rename = "codigoPostal")]
    pub codigo_postal: String,

    #[serde(rename = "idProvincia")]
    pub id_provincia: i32,

    #[serde(rename = "descripcionProvincia")]
    pub descripcion_provincia: String,

    #[serde(rename = "tipoDatoAdicional")]
    pub tipo_dato_adicional: Option<String>,

    #[serde(rename = "datoAdicional")]
    pub dato_adicional: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Persona {
    pub nombre: Nombre,
    pub domicilio: Vec<Domicilio>,
    pub fecha_nacimiento_o_contrato: Option<DateTime<FixedOffset>>,
    pub tipo_persona: TipoPersona,
    pub tipo_forma_juridica: String,
    pub clave: Clave,
    pub documento: Option<Documento>,
    pub actividad_principal: Option<ActividadPrincipal>,
    pub mes_cierre: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Domicilio {
    pub tipo_domicilio: TipoDomicilio,
    pub direccion: Direccion,
    pub localidad: String,
    pub provincia: Provincia,
    pub datos_adicionales: Vec<DatoAdicional>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Direccion {
    pub direccion: String,
    pub calle: Option<String>,
    pub numero: Option<String>,
    pub codigo_postal: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Provincia {
    pub id_provincia: i32,
    pub descripcion_provincia: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DatoAdicional {
    pub tipo_dato_adicional: String,
    pub dato_adicional: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Clave {
    pub id_persona: String,
    pub tipo_clave: TipoClave,
    pub estado_clave: EstadoClave,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Documento {
    pub numero_documento: String,
    pub tipo_documento: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ActividadPrincipal {
    pub id_actividad_principal: String,
    pub descripcion_actividad_principal: String,
    pub periodo_actividad_principal: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Nombre {
    pub nombre: Option<String>,
    pub apellido: Option<String>,
    pub razon_social: Option<String>,
}

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

impl From<PersonaParse> for Persona {
    fn from(value: PersonaParse) -> Self {
        let tipo_persona = value.tipo_persona.as_deref().into();

        let fecha = match value.fecha_nacimiento {
            Some(nacimiento) => Some(nacimiento),
            None => value.fecha_contrato_social,
        };

        Self {
            nombre: Nombre {
                nombre: value.nombre,
                apellido: value.apellido,
                razon_social: value.razon_social,
            },

            domicilio: value.domicilio.into_iter().map(Into::into).collect(),

            fecha_nacimiento_o_contrato: fecha,

            tipo_persona,

            tipo_forma_juridica: value.forma_juridica.unwrap_or_else(|| {
                if tipo_persona == TipoPersona::Fisica {
                    return "Persona Fisica".to_owned();
                }
                "".to_owned()
            }),

            clave: Clave {
                id_persona: value.id_persona.unwrap_or_default(),

                tipo_clave: value.tipo_clave.as_deref().into(),

                estado_clave: value.estado_clave.as_deref().into(),
            },

            documento: match (value.numero_documento, value.tipo_documento) {
                (Some(numero_documento), Some(tipo_documento)) => Some(Documento {
                    numero_documento,
                    tipo_documento,
                }),
                _ => None,
            },

            actividad_principal: match (
                value.id_actividad_principal,
                value.descripcion_actividad_principal,
                value.periodo_actividad_principal,
            ) {
                (Some(id), Some(descripcion), Some(periodo)) => Some(ActividadPrincipal {
                    id_actividad_principal: id.to_string(),
                    descripcion_actividad_principal: descripcion,
                    periodo_actividad_principal: periodo,
                }),

                _ => None,
            },

            mes_cierre: value.mes_cierre.map(i64::from),
        }
    }
}

impl From<DomicilioParse> for Domicilio {
    fn from(value: DomicilioParse) -> Self {
        Self {
            tipo_domicilio: value.tipo_domicilio.as_str().into(),

            direccion: Direccion {
                direccion: value.direccion,
                calle: value.calle,
                numero: value.numero,
                codigo_postal: value.codigo_postal,
            },

            localidad: value
                .localidad
                .unwrap_or_else(|| value.descripcion_provincia.clone()),

            provincia: Provincia {
                id_provincia: value.id_provincia,
                descripcion_provincia: value.descripcion_provincia,
            },

            datos_adicionales: match (value.tipo_dato_adicional, value.dato_adicional) {
                (Some(tipo), Some(dato)) => vec![DatoAdicional {
                    tipo_dato_adicional: tipo,
                    dato_adicional: dato,
                }],
                _ => Vec::new(),
            },
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

impl From<&str> for TipoDomicilio {
    fn from(value: &str) -> Self {
        match value {
            "FISCAL" => Self::Fiscal,
            "LEGAL/REAL" => Self::Legalreal,
            _ => Self::Desconocido,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PersonaCuitRetorno {
    pub parsed: Persona,
    pub answer_xml: String,
}
