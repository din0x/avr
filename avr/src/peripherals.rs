use crate::{
    adc::Adc,
    pins::Pins,
    spi::{Spi, Uninit},
};

macro_rules! peripherals {
    ($($name:ident: $t:ty),* $(,)?) => {
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
                            $name: <$t as $crate::hal::Steal>::steal(),
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
