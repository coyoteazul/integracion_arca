#[cfg(feature = "wsfev1_url")]
mod url;

#[cfg(feature = "wsfev1_fe_cae_solicitar")]
pub mod fe_cae_solicitar;

#[cfg(feature = "wsfev1_dummy")]
mod service_status;
#[cfg(feature = "wsfev1_dummy")]
pub use service_status::service_status;