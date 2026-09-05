#[cfg(feature = "wsaa")]
mod crypto;

pub mod types;

#[cfg(feature = "ws_sr_constancia_inscripcion")]
pub mod ws_sr_constancia_inscripcion;
#[cfg(feature = "ws-sr-padron-a13")]
pub mod ws_sr_padron_a13;
#[cfg(feature = "wsbfev1")]
pub mod wsbfev1;
#[cfg(feature = "wscpe")]
pub mod wscpe;
#[cfg(feature = "wsfev1")]
pub mod wsfev1;
#[cfg(feature = "wsfexv1")]
pub mod wsfexv1;
#[cfg(feature = "wslpg")]
pub mod wslpg;
#[cfg(feature = "wsmtxca")]
pub mod wsmtxca;

mod xml_utils;

#[cfg(feature = "qr_make")]
pub mod qr_make;

#[cfg(feature = "wsaa")]
mod wsaa;
#[cfg(feature = "wsaa")]
pub use wsaa::get_token::{CertKeyPair, ServiceId, TokenArca};
#[cfg(feature = "wsaa")]
pub use wsaa::validate_crt::*;

#[cfg(test)]
pub mod tests;
