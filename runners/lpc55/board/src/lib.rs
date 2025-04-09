#![no_std]

pub use lpc55_hal as hal;

pub mod shared;
pub mod traits;

// board support package
#[cfg(not(any(feature = "lpcxpresso55", feature = "atlkey")))]
compile_error!("Please select one of the board features.");

#[macro_use]
extern crate delog;
generate_macros!();

#[cfg(feature = "lpcxpresso55")]
pub mod lpcxpresso55;
#[cfg(feature = "lpcxpresso55")]
pub use lpcxpresso55 as specifics;

#[cfg(feature = "atlkey")]
pub mod atlkey;
#[cfg(feature = "atlkey")]
pub use atlkey as specifics;

pub use shared::{
    Monotonic,
    Reboot,
};

pub use specifics::{
    button::ThreeButtons,
    led::RgbLed,
};

pub mod clock_controller;
pub mod nfc;
pub mod trussed;

// pub use rgb_led::RgbLed;
