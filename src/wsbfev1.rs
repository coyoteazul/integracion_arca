#[cfg(feature = "wsbfev1_url")]
mod url;

#[cfg(feature = "wsbfev1_dummy")]
mod service_status;
#[cfg(feature = "wsbfev1_dummy")]
pub use service_status::service_status;