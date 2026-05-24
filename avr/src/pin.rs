//! `ATmega16A` pin defnitions.

use core::marker::PhantomData;
use hal::Steal;

pub(crate) mod pin_trait {
    use core::{
        marker::PhantomData,
        ptr::{read_volatile, write_volatile},
    };
    use hal::SetLevel;

    /// Wrapper typed for turning pins into output only.
    // PhantomData<*mut ()> is used so that Output<T>: !Send
    pub struct Out<P: Pin>(P, PhantomData<*mut ()>);

    impl<P: Pin> Out<P> {
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

    impl<P: Pin> SetLevel for Out<P> {
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
    pub trait Pin: private::Sealed + Sized {
        fn into_output(self) -> Out<Self> {
            Out::new(self)
        }

        fn into_spi_device(self) -> hal::Device<Out<Self>> {
            hal::Device::new(self.into_output())
        }

        #[doc(hidden)]
        fn enable_output(&mut self) {
            unsafe {
                write_volatile(Self::DDR, read_volatile(Self::DDR) | Self::MASK);
            }
        }

        #[doc(hidden)]
        fn disable_output(&mut self) {
            unsafe {
                write_volatile(Self::DDR, read_volatile(Self::DDR) & !Self::MASK);
            }
        }
    }

    pub(crate) mod private {
        pub unsafe trait Sealed {
            const PORT: *mut u8;
            const DDR: *mut u8;
            const PIN: *mut u8;
            const MASK: u8;
        }
    }
}

macro_rules! ports {
    ($($Name:ident ($port:ident, $ddr:ident, $pin:ident, $($Pin:ident),*) ),* $(,)?) => {
        $(
            pub struct $Name($(pub $Pin),*);

            impl Steal for $Name {
                unsafe fn steal() -> Self {
                    unsafe {
                        Self($(<$Pin>::steal()),*)
                    }
                }
            }

            $(
                // pins must be !Send, !Sync
                pub struct $Pin(PhantomData<*mut ()>);

                impl Steal for $Pin {
                    unsafe fn steal() -> Self {
                        Self(PhantomData)
                    }
                }

                impl pin_trait::Pin for $Pin {}

                unsafe impl pin_trait::private::Sealed for $Pin {
                    const PORT: *mut u8 = crate::registers::$port;
                    const DDR: *mut u8 = crate::registers::$ddr;
                    const PIN: *mut u8 = crate::registers::$pin;
                    const MASK: u8 = 1 << ${index()};
                }
            )*
        )*
    };
}

ports! {
    PortA(PORTA, DDRA, PINA, Pa0, Pa1, Pa2, Pa3, Pa4, Pa5, Pa6, Pa7),
    PortB(PORTB, DDRB, PINB, Pb0, Pb1, Pb2, Pb3, Pb4, Pb5, Pb6, Pb7),
    PortC(PORTC, DDRC, PINC, Pc0, Pc1, Pc2, Pc3, Pc4, Pc5, Pc6, Pc7),
    PortD(PORTD, DDRD, PIND, Pd0, Pd1, Pd2, Pd3, Pd4, Pd5, Pd6, Pd7),
}
