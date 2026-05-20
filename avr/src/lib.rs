#![feature(asm_experimental_arch)]
#![feature(unsafe_cell_access)]
#![no_std]

mod delay;
mod peripherals;

mod adc;
mod pin;
mod registers;
mod spi;

pub mod interrupts;
pub mod pins;
pub mod state;

pub use adc::{Adc, AdcChannel, Prescaler, Reference};
pub use delay::{delay_ms, delay_us};
pub use pin::{Output, Pin};
pub use spi::Spi;

pub use peripherals::Peripherals;
