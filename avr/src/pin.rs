use core::{
    marker::PhantomData,
    ptr::{read_volatile, write_volatile},
};

use hal::SetLevel;

/// Wrapper typed for turning pins into output only.
// PhantomData<*mut ()> is used so that Output<T>: !Send
pub struct Output<P: Pin>(P, PhantomData<*mut ()>);

impl<P: Pin> Output<P> {
    pub(crate) fn new(mut pin: P) -> Self {
        pin.enable_output();
        Self(pin, PhantomData)
    }

    pub fn toggle(&mut self) {
        unsafe {
            write_volatile(P::PORT, read_volatile(P::PORT) ^ P::MASK);
        }
    }
}

impl<P: Pin> SetLevel for Output<P> {
    fn set_high(&mut self) {
        unsafe {
            write_volatile(P::PORT, read_volatile(P::PORT) | P::MASK);
        }
    }

    fn set_low(&mut self) {
        unsafe {
            write_volatile(P::PORT, read_volatile(P::PORT) & !P::MASK);
        }
    }
}

/// Marks a type as an AVR pin.
pub unsafe trait Pin: Sized {
    const PORT: *mut u8;
    const DDR: *mut u8;
    const PIN: *mut u8;
    const MASK: u8;

    fn enable_output(&mut self) {
        unsafe {
            write_volatile(Self::DDR, read_volatile(Self::DDR) | Self::MASK);
        }
    }

    fn disable_output(&mut self) {
        unsafe {
            write_volatile(Self::DDR, read_volatile(Self::DDR) & !Self::MASK);
        }
    }

    fn into_output(self) -> Output<Self> {
        Output::new(self)
    }
}
