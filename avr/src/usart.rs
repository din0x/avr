use crate::state::Uninit;
use core::marker::PhantomData;
use hal::Steal;

/// Universal Synchronous and Asynchronous serial Receiver and Transmitter.
pub struct Usart<S>(PhantomData<S>);

impl Usart<Uninit> {
    pub fn into_init(self, ubrr: u16) -> Usart<u8> {
        unsafe {
            raw::init(ubrr);
        }

        Usart(PhantomData)
    }
}

impl Steal for Usart<Uninit> {
    unsafe fn steal() -> Self {
        Self(PhantomData)
    }
}

impl Usart<u8> {
    pub fn transmit(&mut self, frame: u8) {
        unsafe {
            raw::transmit(frame);
        }
    }

    pub fn receive(&mut self) -> u8 {
        unsafe { raw::receive() }
    }

    pub fn flush(&mut self) {
        unsafe {
            raw::flush();
        }
    }
}

impl Usart<U9> {
    pub fn transmit(&mut self, frame: U9) {
        unsafe {
            raw::transmit_large(frame.0);
        }
    }

    pub fn receive(&mut self) -> Option<U9> {
        unsafe { raw::receive_large().map(U9) }
    }

    pub fn flush(&mut self) {
        unsafe {
            raw::flush();
        }
    }
}

mod raw {
    use core::ptr::{read_volatile, write_volatile};

    use crate::registers::{UBRRH, UBRRL, UCSRA, UCSRB, UDR};

    const RXC: u8 = 7;

    // const RX: u8 = 7;
    // const TX: u8 = 6;
    const UDRE: u8 = 5;
    const FE: u8 = 4;
    const DOR: u8 = 3;
    const PE: u8 = 2;
    // const U2X: u8 = 1;
    // const MPCM: u8 = 0;

    const TXB8: u8 = 0;

    pub unsafe fn init(ubrr: u16) {
        unsafe {
            /* Set baud rate */
            write_volatile(UBRRH, (ubrr >> 8) as u8);
            write_volatile(UBRRL, (ubrr & 0xff) as u8);

            todo!()
            /* Enable receiver and transmitter */
            // UCSRB = (1<<RXEN)|(1<<TXEN);
            /* Set frame format: 8data, 2stop bit */
            // UCSRC = (1<<URSEL)|(1<<USBS)|(3<<UCSZ0);
        }
    }

    pub unsafe fn transmit(data: u8) {
        unsafe {
            /* Wait for empty transmit buffer */
            while read_volatile(UCSRA) & (1 << UDRE) == 0 {}
        }

        /* Put data into buffer, sends the data */
        unsafe {
            write_volatile(UDR, data);
        }
    }

    pub unsafe fn transmit_large(data: u16) {
        unsafe {
            /* Wait for empty transmit buffer */
            while read_volatile(UCSRA) & (1 << UDRE) == 0 {}
        }

        /* Copy 9th bit to TXB8 */
        unsafe {
            write_volatile(UCSRB, read_volatile(UCSRB) & !(1 << TXB8));
        }

        if data & 0x0100 != 0 {
            unsafe {
                write_volatile(UCSRB, read_volatile(UCSRB) | (1 << TXB8));
            }
        }

        /* Put data into buffer, sends the data */
        unsafe {
            write_volatile(UDR, (data & 0xff) as u8);
        }
    }

    pub unsafe fn receive() -> u8 {
        unsafe {
            /* Wait for data to be received */
            while read_volatile(UCSRA) & (1 << RXC) == 0 {}
            /* Get and return received data from buffer */
            read_volatile(UDR)
        }
    }

    pub unsafe fn receive_large() -> Option<u16> {
        unsafe {
            /* Wait for data to be received */
            while read_volatile(UCSRA) & (1 << RXC) == 0 {}

            /* Get status and 9th bit, then data */
            /* from buffer */
            let status = read_volatile(UCSRA);
            let mut resh = read_volatile(UCSRB);
            let resl = read_volatile(UDR);
            /* If error, return -1 */
            if (status & (1 << FE) | (1 << DOR) | (1 << PE)) != 0 {
                return None;
            }
            /* Filter the 9th bit, then return */
            resh = (resh >> 1) & 0x01;
            return Some((resh << 8) as u16 | resl as u16);
        }
    }

    pub unsafe fn flush() {
        unsafe {
            while (read_volatile(UCSRA) & (1 << RXC)) != 0 {
                read_volatile(UDR);
            }
        }
    }
}

/// Special 9-bit frame used by the USART.
#[derive(Debug, Clone, Copy)]
pub struct U9(pub u16);
