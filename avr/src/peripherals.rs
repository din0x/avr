use crate::{adc::Adc, pins::Pins, spi::Spi, state::Uninit};

macro_rules! peripherals {
    ($($name:ident: $t:ty),* $(,)?) => {
        /// `ATmega16A` peripherals.
        pub struct Peripherals {
            $(
                #[allow(unused)]
                pub $name: $t,
            )*
        }

        impl Peripherals {
            pub unsafe fn steal() -> Self {
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
    pins: Pins,
}
