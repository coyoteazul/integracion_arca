mod url;
pub mod types;

#[cfg(feature = "ws-sr-padron-a13_get_by_dni")]
pub mod get_by_dni;

#[cfg(feature = "ws-sr-padron-a13_get_by_cuit")]
pub mod get_by_cuit;

#[cfg(feature = "ws-sr-padron-a13_dummy")]
mod service_status;

#[cfg(feature = "ws-sr-padron-a13_dummy")]
pub use service_status::service_status;

#[cfg(test)]
pub mod tests;