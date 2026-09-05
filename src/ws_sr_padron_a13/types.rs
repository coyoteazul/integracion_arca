use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::personas::{
    Clave, DatoAdicional, Direccion, Documento, Domicilio, Nombre, Provincia, TipoPersona,
};

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
pub struct ActividadPrincipal {
    pub id_actividad_principal: String,
    pub descripcion_actividad_principal: String,
    pub periodo_actividad_principal: String,
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
                "Desconocido".to_owned()
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

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PersonaCuitRetorno {
    pub parsed: Persona,
    pub answer_xml: String,
}
