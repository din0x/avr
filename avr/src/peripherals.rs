use core::marker::PhantomData;
use crate::{adc::Adc, pins::Pins};

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
                            $name: <$t>::steal(),
                        )*
                    }
                }
            }
        }
    };
}

peripherals! {
    pins: Pins,
    adc: Periph<Adc>,
}

/// A peripheral token, used when explicit initialization is needed.
pub struct Periph<T>(PhantomData<T>);

impl<T> Periph<T> {
    pub unsafe fn steal() -> Self {
        Self(PhantomData)
    }
}
