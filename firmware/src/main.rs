#![no_std]
#![no_main]

use avr::{
    Peripherals, Pin, delay_ms,
    pin::*,
    {Prescaler, Reference},
};
use hal::{SetLevel, Steal};
use hd44780::Lcd;

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

    let mut adc = pd.adc.into_init(Reference::Internal2V56, Prescaler::Div8);
    let mut spi = pd.spi.into_master(pd.b.4, pd.b.5, pd.b.6, pd.b.7);

    let mut lcd = Lcd {
        d7: pd.c.7.into_output(),
        d6: pd.c.6.into_output(),
        d5: pd.c.5.into_output(),
        d4: pd.c.4.into_output(),
        e: pd.d.7.into_output(),
        rs: pd.c.0.into_output(),
    };

    lcd.init();
    lcd.clear();
    ufmt::uwrite!(lcd, "hello world");

    let mut led = pd.d.6.into_output();
    led.set_high();

    let thermometer = pd.a.0;
    let mut accelerometer = adxl345::Adxl345::new(pd.b.3.into_spi_device(), &mut spi);

    loop {
        let (x, y, z) = accelerometer.read_xyz_blocking(&mut spi);

        let voltage = adc.read_blocking(&thermometer);
        let celsius = raw_to_celsius(voltage);

        lcd.clear();
        ufmt::uwrite!(lcd, "x={}y={}z={}\nt={}", x, y, z, celsius);

        led.toggle();
        delay_ms(500);
    }
}

/// Assumes Internal 2.56V ADC 1MHz Div8 prescaler
fn raw_to_celsius(raw: u16) -> u8 {
    (raw >> 2) as u8
}
