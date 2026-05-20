use crate::{
    pin::Pin,
    hal::{SetLevel, Steal},
    pins::{Pb4, Pb5, Pb6, Pb7},
    registers::{SPCR, SPDR, SPSR},
};
use core::ptr::{read_volatile, write_volatile};

pub struct Spi<S>(S);

impl Steal for Spi<Uninit> {
    unsafe fn steal() -> Self {
        Self(Uninit)
    }
}

pub struct Uninit;

pub struct Master;

const SPE: u8 = 6;
const MSTR: u8 = 4;
const SPR0: u8 = 0;

impl Spi<Uninit> {
    pub fn into_master(
        &mut self,
        mut ss: Pb4,
        mut mosi: Pb5,
        mut miso: Pb6,
        mut sck: Pb7,
    ) -> Spi<Master> {
        /* Set MOSI and SCK output, all others input */
        mosi.enable_output();
        sck.enable_output();

        miso.disable_output();
        ss.disable_output();

        /* Enable SPI, Master, set clock rate fck/16 */
        unsafe {
            write_volatile(SPCR, (1 << SPE) | (1 << MSTR) | (1 << SPR0));
        }

        _ = (ss, mosi, miso, sck);
        Spi(Master)
    }
}

impl Spi<Master> {
    pub fn transfer(&mut self, mut pin: impl SetLevel, byte: u8) -> u8 {
        pin.set_low();

        let r = unsafe { transfer_raw(byte) };

        pin.set_high();

        r
    }

    pub fn transfer_batch(&mut self, mut pin: impl SetLevel, batch: &mut [u8]) {
        pin.set_low();

        for byte in batch {
            *byte = unsafe { transfer_raw(*byte) }
        }

        pin.set_high();
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
