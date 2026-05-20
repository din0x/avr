#![no_std]
#![no_main]

use avr::{
    Peripherals,
    adc::{Prescaler, Reference},
    delay_ms,
    pin::Pin,
    lcd::Lcd4,
    pins::*,
};

type Lcd = Lcd4<Pc7, Pc6, Pc5, Pc4, Pd7, Pc0>;

#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> ! {
    let mut lcd = unsafe {
        Lcd {
            d7: Pc7::steal().into_output(),
            d6: Pc6::steal().into_output(),
            d5: Pc5::steal().into_output(),
            d4: Pc4::steal().into_output(),
            e: Pd7::steal().into_output(),
            rs: Pc0::steal().into_output(),
        }
    };

    lcd.init();
    lcd.clear();
    lcd.write("panicked");

    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let pd = unsafe { Peripherals::steal() };

    let pins = pd.pins;

    let mut led = pins.d6.into_output();
    let thermometer = pins.a0;

    let mut adc = pd.adc.into_init(Reference::Internal2V56, Prescaler::Div8);

    let mut lcd = Lcd {
        d7: pins.c7.into_output(),
        d6: pins.c6.into_output(),
        d5: pins.c5.into_output(),
        d4: pins.c4.into_output(),
        e: pins.d7.into_output(),
        rs: pins.c0.into_output(),
    };

    lcd.init();
    lcd.clear();
    lcd.write("hello world");

    delay_ms(1000);

    let mut buf = itoa::Buffer::new();

    loop {
        let voltage = adc.read_blocking(&thermometer);
        let celsius = raw_to_celsius(voltage);

        lcd.clear();
        lcd.write(celsius.fmt(&mut buf));

        led.toggle();
        delay_ms(500);
    }
}

trait Itoa {
    fn fmt(self, buf: &mut itoa::Buffer) -> &str;
}

impl<I: itoa::Integer> Itoa for I {
    #[inline(never)]
    fn fmt(self, buf: &mut itoa::Buffer) -> &str {
        buf.format(self)
    }
}

/// Assumes Internal 2.56V ADC 1MHz Div8 prescaler
fn raw_to_celsius(raw: u16) -> u8 {
    (raw >> 2) as u8
}
