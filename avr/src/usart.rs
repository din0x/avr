use crate::{
    pin::{Pd0, Pd1},
    state::Uninit,
};
use core::{marker::PhantomData, task::Poll};
use hal::Steal;

/// Universal Synchronous and Asynchronous serial Receiver and Transmitter.
pub struct Usart<S>(PhantomData<(S, *mut ())>);

impl Usart<Uninit> {
    pub fn into_init(self, ubrr: u16, rxd: Pd0, txd: Pd1) -> Usart<u8> {
        _ = (rxd, txd);

        unsafe {
            raw::init(ubrr);
            raw::enable_receiver();
            raw::enable_transmitter();
        }

        Usart(PhantomData)
    }
}

impl<S> Usart<S> {
    pub fn into_parts(self) -> (UsartReader<S>, UsartWriter<S>) {
        (UsartReader(PhantomData), UsartWriter(PhantomData))
    }
}

/// Reader part of the USART.
pub struct UsartReader<T>(PhantomData<(T, *mut ())>);

impl UsartReader<u8> {
    pub fn read_blocking(&mut self) -> u8 {
        loop {
            match self.read() {
                Poll::Ready(value) => return value,
                Poll::Pending => {}
            }
        }
    }

    pub fn read(&mut self) -> Poll<u8> {
        unsafe { raw::read() }
    }
}

/// Writer part of the USART.
pub struct UsartWriter<T>(PhantomData<(T, *mut ())>);

impl UsartWriter<u8> {
    pub fn write_blocking(&mut self, frame: u8) {
        loop {
            match self.write(frame) {
                Poll::Ready(()) => return,
                Poll::Pending => {}
            }
        }
    }

    pub fn write(&mut self, frame: u8) -> Poll<()> {
        unsafe { raw::write(frame) }
    }
}

impl Steal for Usart<Uninit> {
    unsafe fn steal() -> Self {
        Self(PhantomData)
    }
}

impl Usart<u8> {
    pub fn read_blocking(&mut self) -> u8 {
        loop {
            match self.read() {
                Poll::Ready(value) => return value,
                Poll::Pending => {}
            }
        }
    }

    pub fn write_blocking(&mut self, frame: u8) {
        loop {
            match self.write(frame) {
                Poll::Ready(()) => return,
                Poll::Pending => {}
            }
        }
    }

    pub fn read(&mut self) -> Poll<u8> {
        unsafe { raw::read() }
    }

    pub fn write(&mut self, frame: u8) -> Poll<()> {
        unsafe { raw::write(frame) }
    }
}

impl Usart<U9> {
    // pub fn write(&mut self, frame: U9) {
    //     unsafe {
    //         raw::transmit_large(frame.0);
    //     }
    // }

    // pub fn read(&mut self) -> Option<U9> {
    //     unsafe { raw::receive_large().map(U9) }
    // }
}

mod raw {
    use core::{
        ptr::{read_volatile, write_volatile},
        task::Poll,
    };

    use crate::registers::{UBRRH, UBRRL, UCSRA, UCSRB, UDR};

    const RXC: u8 = 7;

    // const RX: u8 = 7;
    // const TX: u8 = 6;
    const UDRE: u8 = 5;
    const _FE: u8 = 4;
    const _DOR: u8 = 3;
    const _PE: u8 = 2;
    // const U2X: u8 = 1;
    // const MPCM: u8 = 0;

    const _TXB8: u8 = 0;

    const TXEN: u8 = 3;
    const RXEN: u8 = 4;

    pub unsafe fn enable_receiver() {
        unsafe {
            write_volatile(UCSRB, read_volatile(UCSRB) | 1 << RXEN);
        }
    }

    pub unsafe fn enable_transmitter() {
        unsafe {
            write_volatile(UCSRB, read_volatile(UCSRB) | 1 << TXEN);
        }
    }

    pub unsafe fn init(ubrr: u16) {
        unsafe {
            /* Set baud rate */
            write_volatile(UBRRH, (ubrr >> 8) as u8);
            write_volatile(UBRRL, (ubrr & 0xff) as u8);

            todo!()
            /* Set frame format: 8data, 2stop bit */
            // UCSRC = (1<<URSEL)|(1<<USBS)|(3<<UCSZ0);

            /* Enable receiver and transmitter */
            // UCSRB = (1<<RXEN)|(1<<TXEN);
        }
    }

    pub unsafe fn write(data: u8) -> Poll<()> {
        unsafe {
            /* Wait for empty transmit buffer */
            if read_volatile(UCSRA) & (1 << UDRE) == 0 {
                Poll::Pending
            } else {
                /* Put data into buffer, sends the data */
                Poll::Ready(write_volatile(UDR, data))
            }
        }
    }

    pub unsafe fn read() -> Poll<u8> {
        unsafe {
            /* Wait for data to be received */
            if read_volatile(UCSRA) & (1 << RXC) == 0 {
                Poll::Pending
            } else {
                /* Get and return received data from buffer */
                Poll::Ready(read_volatile(UDR))
            }
        }
    }

    pub unsafe fn _transmit_large(data: u16) {
        unsafe {
            /* Wait for empty transmit buffer */
            while read_volatile(UCSRA) & (1 << UDRE) == 0 {}
        }

        /* Copy 9th bit to TXB8 */
        unsafe {
            write_volatile(UCSRB, read_volatile(UCSRB) & !(1 << _TXB8));
        }

        if data & 0x0100 != 0 {
            unsafe {
                write_volatile(UCSRB, read_volatile(UCSRB) | (1 << _TXB8));
            }
        }

        /* Put data into buffer, sends the data */
        unsafe {
            write_volatile(UDR, (data & 0xff) as u8);
        }
    }

    pub unsafe fn _receive_large() -> Option<u16> {
        unsafe {
            /* Wait for data to be received */
            while read_volatile(UCSRA) & (1 << RXC) == 0 {}

            /* Get status and 9th bit, then data */
            /* from buffer */
            let status = read_volatile(UCSRA);
            let mut resh = read_volatile(UCSRB);
            let resl = read_volatile(UDR);
            /* If error, return -1 */
            if (status & ((1 << _FE) | (1 << _DOR) | (1 << _PE))) != 0 {
                return None;
            }
            /* Filter the 9th bit, then return */
            resh = (resh >> 1) & 0x01;
            Some((resh as u16) << 8 | resl as u16)
        }
    }
}

/// Special 9-bit frame used by the USART.
#[derive(Debug, Clone, Copy)]
pub struct U9(pub u16);
