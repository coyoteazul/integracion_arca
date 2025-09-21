#[cfg(feature = "wsfexv1_url")]
mod url;

#[cfg(feature = "wsfexv1_dummy")]
mod service_status;
#[cfg(feature = "wsfexv1_dummy")]
pub use service_status::service_status;