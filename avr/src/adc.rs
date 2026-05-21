use crate::{
    pin::*,
    registers::{ADCH, ADCL, ADCSRA, ADMUX},
    state::{Init, Uninit},
};
use core::ptr::{read_volatile, write_volatile};
use hal::Steal;

/// Analog to Digital Converter.
pub struct Adc<S> {
    _state: S,
    reference: Reference,
    _prescaler: Prescaler,
}

impl Steal for Adc<Uninit> {
    unsafe fn steal() -> Self {
        Self {
            _state: Uninit,
            reference: Reference::ARef,
            _prescaler: Prescaler::Div2,
        }
    }
}

impl Adc<Uninit> {
    #[inline(never)]
    pub fn into_init(self, reference: Reference, prescaler: Prescaler) -> Adc<Init> {
        unsafe {
            // Set reference (MUX bits cleared → channel 0 by default)
            write_volatile(ADMUX, reference as u8);

            // Enable ADC + prescaler
            write_volatile(ADCSRA, ADEN | prescaler as u8);

            // Discard the first conversion (the datasheet recommends this
            // after enabling the ADC, as the first result may be inaccurate).
            write_volatile(ADCSRA, read_volatile(ADCSRA) | ADSC);
            while read_volatile(ADCSRA) & ADSC > 0 {}
        }

        Adc {
            _state: Init,
            reference,
            _prescaler: prescaler,
        }
    }
}

impl Adc<Init> {
    #[inline(always)]
    pub fn read_blocking<P: AdcChannel>(&mut self, pin: &P) -> u16 {
        _ = pin;
        self.read_blocking_impl(P::CHANNEL)
    }

    #[inline(never)]
    fn read_blocking_impl(&mut self, channel: u8) -> u16 {
        unsafe {
            // Update channel, preserve reference bits
            write_volatile(ADMUX, self.reference as u8 | (channel & 0x07));

            // Start conversion
            write_volatile(ADCSRA, read_volatile(ADCSRA) | ADSC);

            // Wait for ADSC to clear
            while read_volatile(ADCSRA) & ADSC > 0 {}

            // ADCL must be read first — this locks both registers until ADCH
            // is read, guaranteeing a consistent 16-bit capture.
            let low = read_volatile(ADCL) as u16;
            let high = read_volatile(ADCH) as u16;

            (high << 8) | low
        }
    }
}

/// Marks a pin as a valid source for the Analog-to-Digital Converter.
pub trait AdcChannel: private::Sealed {
    const CHANNEL: u8;
}

mod private {
    pub unsafe trait Sealed {}
}

macro_rules! impl_adc_channel {
    ($($pin:ident => $channel:literal),* $(,)?) => {
        $(
            impl AdcChannel for $pin {
                const CHANNEL: u8 = $channel;
            }

            unsafe impl private::Sealed for $pin {}
        )*
    };
}

impl_adc_channel! {
    Pa0 => 0,
    Pa1 => 1,
    Pa2 => 2,
    Pa3 => 3,
    Pa4 => 4,
    Pa5 => 5,
    Pa6 => 6,
    Pa7 => 7,
}

const REFS1: u8 = 1 << 7;
const REFS0: u8 = 1 << 6;
const ADEN: u8 = 1 << 7;
const ADSC: u8 = 1 << 6;

/// Reference voltage used by the Analog-to-Digital Converter.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Reference {
    ARef = 0,
    AVcc = REFS0,
    Internal2V56 = REFS1 | REFS0,
}

/// Prescaler used by the Analog-to-Digital Converter.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Prescaler {
    Div2 = 0b001,
    Div4 = 0b010,
    Div8 = 0b011,
    Div16 = 0b100,
    Div32 = 0b101,
    Div64 = 0b110,
    Div128 = 0b111,
}
