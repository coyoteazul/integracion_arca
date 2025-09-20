#[cfg(feature = "wsaa")]
pub mod errors;
#[cfg(feature = "wsaa")]
pub(super) mod enums;

#[cfg(feature = "dummy_type")]
mod dummy_result;
#[cfg(feature = "dummy_type")]
pub use dummy_result::FEDummyResult;