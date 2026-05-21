use crate::{Usart, adc::Adc, pin::Pins, spi::Spi, state::Uninit};

macro_rules! peripherals {
    ($($name:ident: $t:ty),* $(,)?) => {
        /// `ATmega16A` peripherals.
        pub struct Peripherals {
            $(
                pub $name: $t,
            )*
        }

        impl ::hal::Steal for Peripherals {
            unsafe fn steal() -> Self {
                unsafe {
                    Self {
                        $(
                            $name: <$t as ::hal::Steal>::steal(),
                        )*
                    }
                }
            }
        }
    };
}

peripherals! {
    adc: Adc<Uninit>,
    spi: Spi<Uninit>,
    usart: Usart<Uninit>,
    pins: Pins,
}
