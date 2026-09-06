use serde::Deserialize;

use crate::wsfev1::fe_cae_solicitar::types::{ErrorsWrapper, EventsWrapper, Wsfev1Obs};

/// Resultado de FECompUltimoAutorizado: el ultimo numero de comprobante autorizado para
/// el punto de venta / tipo de comprobante consultado. El proximo numero valido para
/// FECAESolicitar es `cbte_nro + 1`.
#[derive(Debug)]
pub struct UltimoAutorizadoRetorno {
    pub pto_vta: i64,
    pub cbte_tipo: i64,
    pub cbte_nro: i64,
    pub obs: Vec<Wsfev1Obs>,
    /// El request enviado a ARCA, con el bloque <ar:Auth> reemplazado por un comentario.
    pub sent_xml: String,
    pub received_xml: String,
}

// ---------- tipo interno: wire format (FECompUltimoAutorizadoResult) ----------
//
// El WSDL declara PtoVta/CbteTipo/CbteNro como minOccurs="1", pero -consistente con el
// resto del modulo- no confiamos ciegamente en eso: si alguno faltara, preferimos
// detectarlo explicitamente (ver fecomp_ultimo_autorizado.rs) en vez de que quick_xml
// tire un error de parseo generico.

#[derive(Debug, Deserialize, Default)]
pub(crate) struct FeCompUltimoAutorizadoParse {
    #[serde(rename = "PtoVta")]
    pub pto_vta: Option<i32>,
    #[serde(rename = "CbteTipo")]
    pub cbte_tipo: Option<i32>,
    #[serde(rename = "CbteNro")]
    pub cbte_nro: Option<i32>,
    #[serde(rename = "Errors", default)]
    pub errors: Option<ErrorsWrapper>,
    #[serde(rename = "Events", default)]
    pub events: Option<EventsWrapper>,
}