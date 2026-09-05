pub mod types;
mod url;

#[cfg(feature = "ws_sr_constancia_inscripcion_get_by_cuit")]
pub mod get_by_cuit;

#[cfg(feature = "ws_sr_constancia_inscripcion_dummy")]
mod service_status;

#[cfg(feature = "ws_sr_constancia_inscripcion_dummy")]
pub use service_status::service_status;

#[cfg(test)]
pub mod tests;
