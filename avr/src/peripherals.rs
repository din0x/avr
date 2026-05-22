use crate::{
    Timer0, Usart, adc::Adc, pin::{PortA, PortB, PortC, PortD}, spi::Spi, state::Uninit
};

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
    tc0: Timer0<Uninit>,
    a: PortA,
    b: PortB,
    c: PortC,
    d: PortD,
}
