use core::ptr::{read_volatile, write_volatile};

// Sets the pin into write mode.
pub struct Output<P: Gpio>(P);

impl<P: Gpio> Output<P> {
    pub(crate) fn new(pin: P) -> Self {
        unsafe {
            write_volatile(P::DDR, read_volatile(P::DDR) | P::MASK);
        }

        Self(pin)
    }

    pub fn toggle(&mut self) {
        unsafe {
            write_volatile(P::PORT, read_volatile(P::PORT) ^ P::MASK);
        }
    }

    pub fn write(&mut self, bit: bool) {
        match bit {
            true => self.set_high(),
            false => self.set_low(),
        }
    }

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

pub unsafe trait Gpio: Sized {
    const PORT: *mut u8;
    const DDR: *mut u8;
    #[allow(unused)]
    const PIN: *mut u8;
    const MASK: u8;

    fn into_output(self) -> Output<Self> {
        Output::new(self)
    }
}
