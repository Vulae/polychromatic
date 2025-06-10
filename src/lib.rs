use thiserror::Error;

mod color;
mod defs;
pub mod device;
pub mod effect;

pub use color::*;
pub use device::*;
pub use effect::*;

pub(crate) const FPS_RANGE: std::ops::RangeInclusive<u32> = 1..=80;

#[derive(Debug, Error)]
pub enum PolychromaticError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("{0:?} doesn't support customizable lighting")]
    DeviceUnsupportedEffects(Device),
    #[error("Invalid FPS value {0} must be in range 1..=80")]
    InvalidFPS(u32),
}
