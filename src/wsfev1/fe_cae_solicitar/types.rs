use std::fmt;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::types::tipo_rg1415::TipoRG1415;

// ---------- tipos publicos: request ----------

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Comprobante {
    pub id_factura: i64,
    pub cabezal: ComprobCabezal,
    pub cliente: ComprobCliente,
    pub valores: ComprobValores,

    pub comprob_asociados: Option<Vec<ComprobAsoc>>,
    pub periodo_asociado: Option<ComprobPeriodo>,
    pub opcionales: Option<Vec<ComprobOpcionales>>,
    pub actividades: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ComprobCabezal {
    pub punto_venta: i64,
    pub num_documento: i64,
    pub tipo_rg1415: TipoRG1415,
    ///1:Productos, 2:Servicios, 3:Ambos
    pub concepto: TipoVenta,
    pub fecha_emision: NaiveDate,
    pub moneda: String,
    pub cotizacion: f64,
    pub cancela_misma_moneda: bool,
    pub servicio_desde: Option<NaiveDate>,
    pub servicio_hasta: Option<NaiveDate>,
    pub venci_pago: Option<NaiveDate>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ComprobCliente {
    pub tipo_doc: i64,
    pub documento: i64,
    pub cond_iva: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ComprobValores {
    pub val_total: f64,
    pub val_nogravado: f64,
    pub val_gravado: f64,
    pub val_exento: f64,
    pub val_iva: f64,
    pub val_otros_trib: f64,
    pub tributos: Option<Vec<ComprobTributos>>,
    pub alicuotas_iva: Option<Vec<ComprobIVA>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ComprobAsoc {
    pub punto_venta: i64,
    pub num_documento: i64,
    pub tipo_rg1415: TipoRG1415,
    pub fecha_emision: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ComprobTributos {
    pub id_tributo: i64,
    pub desc: String,
    pub base: f64,
    pub alicuota: f64,
    pub importe: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ComprobIVA {
    pub id_alicuota: i64,
    pub base: f64,
    pub importe: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ComprobPeriodo {
    pub fecha_desde: NaiveDate,
    pub fecha_hasta: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ComprobOpcionales {
    pub id: String,
    pub valor: String,
}

// ---------- tipos publicos: response ----------

/// Resultado completo de un pedido de CAE: el resultado de negocio (aprobado/rechazado),
/// mas el XML enviado (sin datos de Auth) y el XML crudo recibido, para diagnostico/logging.
#[derive(Debug, Serialize, Deserialize)]
pub struct FecaeRetorno {
    pub resultado: Result<Wsfev1Ok, Wsfev1Rechazo>,
    /// El request enviado a ARCA, con el bloque <ar:Auth> reemplazado por un comentario.
    pub sent_xml: String,
    pub received_xml: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wsfev1Ok {
    pub cae: String,
    pub vcto: NaiveDate,
    pub obs: Vec<Wsfev1Obs>,
}

/// Rechazo de negocio (Resultado = "R"). No es un error de transporte ni un SOAP Fault:
/// ARCA proceso el pedido correctamente y decidio no otorgar el CAE.
#[derive(Debug, Serialize, Deserialize)]
pub struct Wsfev1Rechazo {
    pub obs: Vec<Wsfev1Obs>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Wsfev1Obs {
    pub code: String,
    pub msg: String,
}

// ---------- tipos internos: wire format (FECAESolicitarResult) ----------
//
// Reflejan 1:1 el esquema de FECAEResponse en el WSDL. Todo Option/Vec-con-default
// porque el WSDL declara casi todo con minOccurs="0", y preferimos no asumir presencia
// garantizada aunque la tabla en prosa del manual diga "Obligatorio: S" (ya nos paso en
// otros servicios de ARCA que la prosa y el comportamiento real no coinciden).

#[derive(Debug, Deserialize, Default)]
pub(crate) struct FecaeResponseParse {
    #[serde(rename = "FeCabResp")]
    pub cab: Option<FeCabRespParse>,
    #[serde(rename = "FeDetResp", default)]
    pub det: FeDetRespWrapper,
    #[serde(rename = "Events", default)]
    pub events: Option<EventsWrapper>,
    #[serde(rename = "Errors", default)]
    pub errors: Option<ErrorsWrapper>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct FeCabRespParse {
    #[serde(rename = "Cuit")]
    pub cuit: Option<i64>,
    #[serde(rename = "PtoVta")]
    pub pto_vta: Option<i32>,
    #[serde(rename = "CbteTipo")]
    pub cbte_tipo: Option<i32>,
    #[serde(rename = "FchProceso")]
    pub fch_proceso: Option<String>,
    #[serde(rename = "CantReg")]
    pub cant_reg: Option<i32>,
    #[serde(rename = "Resultado")]
    pub resultado: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct FeDetRespWrapper {
    #[serde(rename = "FECAEDetResponse", default)]
    pub items: Vec<FeDetRespParse>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct FeDetRespParse {
    #[serde(rename = "Concepto")]
    pub concepto: Option<i32>,
    #[serde(rename = "DocTipo")]
    pub doc_tipo: Option<i32>,
    #[serde(rename = "DocNro")]
    pub doc_nro: Option<i64>,
    #[serde(rename = "CbteDesde")]
    pub cbte_desde: Option<i64>,
    #[serde(rename = "CbteHasta")]
    pub cbte_hasta: Option<i64>,
    #[serde(rename = "CbteFch")]
    pub cbte_fch: Option<String>,
    #[serde(rename = "Resultado")]
    pub resultado: Option<String>,
    #[serde(rename = "CAE")]
    pub cae: Option<String>,
    #[serde(rename = "CAEFchVto")]
    pub cae_fch_vto: Option<String>,
    #[serde(rename = "Observaciones", default)]
    pub observaciones: ObservacionesWrapper,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct ObservacionesWrapper {
    #[serde(rename = "Obs", default)]
    pub items: Vec<CodeMsgParse>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct EventsWrapper {
    #[serde(rename = "Evt", default)]
    pub items: Vec<CodeMsgParse>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct ErrorsWrapper {
    #[serde(rename = "Err", default)]
    pub items: Vec<CodeMsgParse>,
}

/// Obs, Evt y Err comparten exactamente la misma forma (Code/Msg) en el WSDL.
#[derive(Debug, Deserialize, Default)]
pub(crate) struct CodeMsgParse {
    #[serde(rename = "Code")]
    pub code: Option<i32>,
    #[serde(rename = "Msg")]
    pub msg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum TipoVenta {
    Productos = 1,
    Servicios = 2,
    Ambos = 3,
}

impl From<i8> for TipoVenta {
		fn from(value: i8) -> Self {
				match value {
						1 => TipoVenta::Productos,
						2 => TipoVenta::Servicios,
						3 => TipoVenta::Ambos,
						_ => TipoVenta::Productos, // default
				}
		}
}

impl fmt::Display for TipoVenta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}
