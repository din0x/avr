use crate::{
    Pin,
    pin::{Pb4, Pb5, Pb6, Pb7},
    registers::{SPCR, SPDR, SPSR},
    state::{Master, Uninit},
};
use core::ptr::{read_volatile, write_volatile};
use hal::{SetLevel, Steal};

/// Serial Peripheral Interface.
pub struct Spi<S>(S);

const SPE: u8 = 6; // SPI Enable
const MSTR: u8 = 4; // Master mode
const CPOL: u8 = 3; // Clock polarity
const CPHA: u8 = 2; // Clock phase
const SPR0: u8 = 0; // Clock prescaler bit 0

impl Spi<Uninit> {
    pub fn into_master(self, ss: Pb4, mut mosi: Pb5, mut miso: Pb6, mut sck: Pb7) -> Spi<Master> {
        /* Set MOSI and SCK output, all others input */
        mosi.enable_output();
        sck.enable_output();

        miso.disable_output();
        ss.into_output().set_high();

        /* Enable SPI, Master, set clock rate fck/16 */
        unsafe {
            // write_volatile(SPCR, (1 << SPE) | (1 << MSTR) | (1 << SPR0));

            write_volatile(
                SPCR,
                (1 << SPE) | (1 << MSTR) | (1 << SPR0) | (1 << CPOL) | (1 << CPHA),
            );
        }

        _ = (mosi, miso, sck);
        Spi(Master)
    }
}

impl hal::Transfer for Spi<Master> {
    fn transfer_batch(&mut self, device: &mut hal::Device<impl SetLevel>, batch: &mut [u8]) {
        device.0.set_low();

        for byte in batch {
            *byte = unsafe { transfer_raw(*byte) }
        }

        device.0.set_high();
    }
}

impl Steal for Spi<Uninit> {
    unsafe fn steal() -> Self {
        Self(Uninit)
    }
}

#[inline(never)]
unsafe fn transfer_raw(byte: u8) -> u8 {
    const SPIF: u8 = 7;

    unsafe {
        /* Start transmission */
        write_volatile(SPDR, byte);
        /* Wait for transmission complete */
        while read_volatile(SPSR) & (1 << SPIF) == 0 {}

        read_volatile(SPDR)
    }
}
