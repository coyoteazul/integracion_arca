use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::personas::{
    Clave, DatoAdicional, Direccion, Domicilio, Nombre, Provincia, TipoPersona,
};

// ---------- raw wire format (internal only) ----------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct PersonaA5Parse {
    #[serde(rename = "datosGenerales")]
    pub datos_generales: Option<DatosGeneralesA5Parse>,

    #[serde(rename = "datosRegimenGeneral")]
    pub datos_regimen_general: Option<DatosRegimenGeneralA5Parse>,

    #[serde(rename = "datosMonotributo")]
    pub datos_monotributo: Option<DatosMonotributoA5Parse>,

    #[serde(rename = "errorConstancia")]
    pub error_constancia: Option<ErrorConstanciaA5Parse>,

    #[serde(rename = "errorRegimenGeneral")]
    pub error_regimen_general: Option<ErrorRegimenGeneralA5Parse>,

    #[serde(rename = "errorMonotributo")]
    pub error_monotributo: Option<ErrorMonotributoA5Parse>,

    pub metadata: Option<MetadataA5Parse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct DatosGeneralesA5Parse {
    #[serde(rename = "idPersona")]
    pub id_persona: Option<i64>,
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
    #[serde(rename = "esSucesion")]
    pub es_sucesion: Option<String>,
    #[serde(rename = "mesCierre")]
    pub mes_cierre: Option<i32>,
    #[serde(rename = "fechaContratoSocial")]
    pub fecha_contrato_social: Option<DateTime<FixedOffset>>,
    pub dependencia: Option<DependenciaA5Parse>,
    #[serde(rename = "domicilioFiscal")]
    pub domicilio_fiscal: Option<DomicilioA5Parse>,
    #[serde(default)]
    pub caracterizacion: Vec<CaracterizacionA5Parse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct DatosRegimenGeneralA5Parse {
    #[serde(default)]
    pub impuesto: Vec<ImpuestoA5Parse>,
    #[serde(rename = "categoriaAutonomo", default)]
    pub categoria_autonomo: Vec<CategoriaA5Parse>,
    #[serde(default)]
    pub regimen: Vec<RegimenA5Parse>,
    #[serde(default)]
    pub actividad: Vec<ActividadA5Parse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct DatosMonotributoA5Parse {
    pub impuesto: Option<ImpuestoA5Parse>,
    #[serde(rename = "actividadMonotributista")]
    pub actividad_monotributista: Option<ActividadA5Parse>,
    #[serde(rename = "categoriaMonotributo")]
    pub categoria_monotributo: Option<CategoriaA5Parse>,
    #[serde(rename = "componenteDeSociedad", default)]
    pub componente_de_sociedad: Vec<ComponenteA5Parse>,
    #[serde(default)]
    pub actividad: Vec<ActividadA5Parse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct ImpuestoA5Parse {
    #[serde(rename = "descripcionImpuesto")]
    pub descripcion_impuesto: Option<String>,
    #[serde(rename = "estadoImpuesto")]
    pub estado_impuesto: Option<String>,
    #[serde(rename = "idImpuesto")]
    pub id_impuesto: Option<i32>,
    pub motivo: Option<String>,
    pub periodo: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct RegimenA5Parse {
    #[serde(rename = "descripcionRegimen")]
    pub descripcion_regimen: Option<String>,
    #[serde(rename = "idRegimen")]
    pub id_regimen: Option<i32>,
    #[serde(rename = "idImpuesto")]
    pub id_impuesto: Option<i32>,
    pub periodo: Option<i32>,
    #[serde(rename = "tipoRegimen")]
    pub tipo_regimen: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct CategoriaA5Parse {
    #[serde(rename = "descripcionCategoria")]
    pub descripcion_categoria: Option<String>,
    #[serde(rename = "idCategoria")]
    pub id_categoria: Option<i32>,
    #[serde(rename = "idImpuesto")]
    pub id_impuesto: Option<i32>,
    pub periodo: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct DomicilioA5Parse {
    #[serde(rename = "tipoDomicilio")]
    pub tipo_domicilio: Option<String>,
    pub direccion: Option<String>,
    pub localidad: Option<String>,
    #[serde(rename = "codPostal")]
    pub cod_postal: Option<String>,
    #[serde(rename = "idProvincia")]
    pub id_provincia: Option<i32>,
    #[serde(rename = "descripcionProvincia")]
    pub descripcion_provincia: Option<String>,
    #[serde(rename = "tipoDatoAdicional")]
    pub tipo_dato_adicional: Option<String>,
    #[serde(rename = "datoAdicional")]
    pub dato_adicional: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct DependenciaA5Parse {
    #[serde(rename = "idDependencia")]
    pub id_dependencia: Option<i32>,
    #[serde(rename = "descripcionDependencia")]
    pub descripcion_dependencia: Option<String>,
    #[serde(rename = "codPostal")]
    pub cod_postal: Option<String>,
    pub direccion: Option<String>,
    #[serde(rename = "idProvincia")]
    pub id_provincia: Option<i32>,
    #[serde(rename = "descripcionProvincia")]
    pub descripcion_provincia: Option<String>,
    pub localidad: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct ComponenteA5Parse {
    #[serde(rename = "idPersona")]
    pub id_persona: Option<i64>,
    pub nombre: Option<String>,
    pub apellido: Option<String>,
    #[serde(rename = "tipoComponente")]
    pub tipo_componente: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct MetadataA5Parse {
    #[serde(rename = "fechaHora")]
    pub fecha_hora: Option<DateTime<FixedOffset>>,
    pub servidor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct ActividadA5Parse {
    #[serde(rename = "descripcionActividad")]
    pub descripcion_actividad: Option<String>,
    #[serde(rename = "idActividad")]
    pub id_actividad: Option<i64>,
    pub nomenclador: Option<i32>,
    pub orden: Option<i32>,
    pub periodo: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct CaracterizacionA5Parse {
    #[serde(rename = "descripcionCaracterizacion")]
    pub descripcion_caracterizacion: Option<String>,
    #[serde(rename = "fechaSolicitud")]
    pub fecha_solicitud: Option<i32>,
    #[serde(rename = "idCaracterizacion")]
    pub id_caracterizacion: Option<i32>,
    pub periodo: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct ErrorConstanciaA5Parse {
    pub apellido: Option<String>,
    #[serde(default)]
    pub error: Vec<String>,
    #[serde(rename = "idPersona")]
    pub id_persona: Option<i64>,
    pub nombre: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct ErrorMonotributoA5Parse {
    #[serde(default)]
    pub error: Vec<String>,
    pub mensaje: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct ErrorRegimenGeneralA5Parse {
    #[serde(default)]
    pub error: Vec<String>,
    pub mensaje: Option<String>,
}

// ---------- public API ----------

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ConstanciaInscripcion {
    pub nombre: Nombre,
    pub clave: Clave,
    pub tipo_persona: TipoPersona,
    pub es_sucesion: bool,
    pub mes_cierre: Option<i32>,
    pub fecha_contrato_social: Option<DateTime<FixedOffset>>,
    pub domicilio_fiscal: Option<Domicilio>,
    pub caracterizaciones: Vec<Caracterizacion>,
    pub regimen_general: Option<RegimenGeneral>,
    pub monotributo: Option<Monotributo>,
    /// Mensajes de error devueltos por ARCA (errorConstancia/errorRegimenGeneral/errorMonotributo),
    /// aplanados. Un CUIT inexistente, por ejemplo, llega acá en vez de como Err.
    pub errores: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RegimenGeneral {
    pub impuestos: Vec<Impuesto>,
    pub categorias_autonomo: Vec<Categoria>,
    pub regimenes: Vec<Regimen>,
    pub actividades: Vec<Actividad>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Monotributo {
    pub impuesto: Option<Impuesto>,
    pub actividad_monotributista: Option<Actividad>,
    pub categoria: Option<Categoria>,
    pub actividades: Vec<Actividad>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Actividad {
    pub id_actividad: i64,
    pub descripcion_actividad: String,
    pub nomenclador: Option<i32>,
    pub orden: Option<i32>,
    pub periodo: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Impuesto {
    pub id_impuesto: i32,
    pub descripcion_impuesto: String,
    pub estado_impuesto: String,
    // Genuinamente opcional: confirmado ausente en respuestas reales (ver Anexo del manual).
    pub motivo: Option<String>,
    pub periodo: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Regimen {
    pub id_regimen: i32,
    pub descripcion_regimen: String,
    pub id_impuesto: i32,
    pub periodo: i32,
    // Confirmado ausente en algunos regímenes reales (ver log de "PARTICIPACIONES SOCIETARIAS").
    pub tipo_regimen: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Categoria {
    pub id_categoria: i32,
    pub descripcion_categoria: String,
    pub id_impuesto: i32,
    pub periodo: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Caracterizacion {
    pub id_caracterizacion: i32,
    pub descripcion_caracterizacion: String,
    pub periodo: Option<i32>,
    pub fecha_solicitud: Option<i32>,
}

// ---------- conversions ----------

impl From<PersonaA5Parse> for ConstanciaInscripcion {
    fn from(value: PersonaA5Parse) -> Self {
        let dg = value.datos_generales;

        let mut errores = Vec::new();
        if let Some(e) = &value.error_regimen_general {
            errores.extend(e.error.clone());
            errores.extend(e.mensaje.clone());
        }
        if let Some(e) = &value.error_monotributo {
            errores.extend(e.error.clone());
            errores.extend(e.mensaje.clone());
        }
        if let Some(e) = &value.error_constancia {
            errores.extend(e.error.clone());
        }

        let (nombre, apellido, id_persona) = match (dg.as_ref(), value.error_constancia.as_ref()) {
            (Some(dg), _) => (dg.nombre.clone(), dg.apellido.clone(), dg.id_persona),
            (None, Some(err)) => (err.nombre.clone(), err.apellido.clone(), err.id_persona),
            (None, None) => (None, None, None),
        };

        let dg = dg.unwrap_or_default();

        Self {
            nombre: Nombre {
                nombre,
                apellido,
                razon_social: dg.razon_social,
            },
            clave: Clave {
                id_persona: id_persona.map(|c| c.to_string()).unwrap_or_default(),
                tipo_clave: dg.tipo_clave.as_deref().into(),
                estado_clave: dg.estado_clave.as_deref().into(),
            },
            tipo_persona: dg.tipo_persona.as_deref().into(),
            es_sucesion: dg.es_sucesion.as_deref() == Some("SI"),
            mes_cierre: dg.mes_cierre,
            fecha_contrato_social: dg.fecha_contrato_social,
            domicilio_fiscal: dg.domicilio_fiscal.map(Into::into),
            caracterizaciones: dg.caracterizacion.into_iter().map(Into::into).collect(),
            regimen_general: value.datos_regimen_general.map(Into::into),
            monotributo: value.datos_monotributo.map(Into::into),
            errores,
        }
    }
}

impl From<DomicilioA5Parse> for Domicilio {
    fn from(value: DomicilioA5Parse) -> Self {
        let descripcion_provincia = value.descripcion_provincia.unwrap_or_default();
        Self {
            tipo_domicilio: value.tipo_domicilio.as_deref().into(),
            direccion: Direccion {
                direccion: value.direccion.unwrap_or_default(),
                calle: None,
                numero: None,
                codigo_postal: value.cod_postal.unwrap_or_default(),
            },
            localidad: value
                .localidad
                .unwrap_or_else(|| descripcion_provincia.clone()),
            provincia: Provincia {
                id_provincia: value.id_provincia.unwrap_or_default(),
                descripcion_provincia,
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

impl From<DatosRegimenGeneralA5Parse> for RegimenGeneral {
    fn from(value: DatosRegimenGeneralA5Parse) -> Self {
        Self {
            impuestos: value.impuesto.into_iter().map(Into::into).collect(),
            categorias_autonomo: value
                .categoria_autonomo
                .into_iter()
                .map(Into::into)
                .collect(),
            regimenes: value.regimen.into_iter().map(Into::into).collect(),
            actividades: value.actividad.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<DatosMonotributoA5Parse> for Monotributo {
    fn from(value: DatosMonotributoA5Parse) -> Self {
        Self {
            impuesto: value.impuesto.map(Into::into),
            actividad_monotributista: value.actividad_monotributista.map(Into::into),
            categoria: value.categoria_monotributo.map(Into::into),
            actividades: value.actividad.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ActividadA5Parse> for Actividad {
    fn from(value: ActividadA5Parse) -> Self {
        Self {
            id_actividad: value.id_actividad.unwrap_or_default(),
            descripcion_actividad: value.descripcion_actividad.unwrap_or_default(),
            nomenclador: value.nomenclador,
            orden: value.orden,
            periodo: value.periodo,
        }
    }
}

impl From<ImpuestoA5Parse> for Impuesto {
    fn from(value: ImpuestoA5Parse) -> Self {
        Self {
            id_impuesto: value.id_impuesto.unwrap_or_default(),
            descripcion_impuesto: value.descripcion_impuesto.unwrap_or_default(),
            estado_impuesto: value.estado_impuesto.unwrap_or_default(),
            motivo: value.motivo,
            periodo: value.periodo.unwrap_or_default(),
        }
    }
}

impl From<RegimenA5Parse> for Regimen {
    fn from(value: RegimenA5Parse) -> Self {
        Self {
            id_regimen: value.id_regimen.unwrap_or_default(),
            descripcion_regimen: value.descripcion_regimen.unwrap_or_default(),
            id_impuesto: value.id_impuesto.unwrap_or_default(),
            periodo: value.periodo.unwrap_or_default(),
            tipo_regimen: value.tipo_regimen,
        }
    }
}

impl From<CategoriaA5Parse> for Categoria {
    fn from(value: CategoriaA5Parse) -> Self {
        Self {
            id_categoria: value.id_categoria.unwrap_or_default(),
            descripcion_categoria: value.descripcion_categoria.unwrap_or_default(),
            id_impuesto: value.id_impuesto.unwrap_or_default(),
            periodo: value.periodo.unwrap_or_default(),
        }
    }
}

impl From<CaracterizacionA5Parse> for Caracterizacion {
    fn from(value: CaracterizacionA5Parse) -> Self {
        Self {
            id_caracterizacion: value.id_caracterizacion.unwrap_or_default(),
            descripcion_caracterizacion: value.descripcion_caracterizacion.unwrap_or_default(),
            periodo: value.periodo,
            fecha_solicitud: value.fecha_solicitud,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ConstanciaInscripcionRetorno {
    pub parsed: ConstanciaInscripcion,
    pub answer_xml: String,
}
