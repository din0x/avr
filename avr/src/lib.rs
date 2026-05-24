#![feature(asm_experimental_arch)]
#![feature(unsafe_cell_access)]
#![feature(macro_metavar_expr)]
#![no_std]

mod adc;
mod delay;
mod peripherals;
mod registers;
mod spi;
mod timer;
mod usart;

pub mod interrupts;
pub mod pin;
pub mod state;

pub use adc::{Adc, AdcChannel, Prescaler, Reference};
pub use delay::{delay_ms, delay_us};
pub use peripherals::Peripherals;
#[doc(inline)]
pub use pin::pin_trait::{Out, Pin};
pub use spi::Spi;
pub use timer::{Normal, Timer0};
pub use usart::{U9, Usart, UsartReader, UsartWriter};
