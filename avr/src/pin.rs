use core::ptr::{read_volatile, write_volatile};

// Sets the pin into write mode.
pub struct Output<P: Pin>(P);

impl<P: Pin> Output<P> {
    pub(crate) fn new(mut pin: P) -> Self {
        pin.enable_output();
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

pub unsafe trait Pin: Sized {
    const PORT: *mut u8;
    const DDR: *mut u8;
    #[allow(unused)]
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
