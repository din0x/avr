use core::marker::PhantomData;
use crate::{pin::Pin, hal::Steal};

macro_rules! pins {
    (
        $(
            $name:ident :
            $ty:ident => ($port:ident, $ddr:ident, $pin:ident, $bit:literal)
        ),* $(,)?
    ) => {
        pub struct Pins {
            $(
                pub $name: $ty,
            )*
        }

        impl Steal for Pins {
            unsafe fn steal() -> Self {
                unsafe {
                    Self {
                        $(
                            $name: <$ty>::steal(),
                        )*
                    }
                }
            }
        }

        $(
            // pins must be !Send, !Sync
            pub struct $ty(PhantomData<*mut ()>);

            impl $ty {
                #[allow(unused)]
                pub unsafe fn steal() -> Self {
                    Self(PhantomData)
                }
            }

            unsafe impl Pin for $ty {
                const PORT: *mut u8 = crate::registers::$port;
                const DDR: *mut u8 = crate::registers::$ddr;
                const PIN: *mut u8 = crate::registers::$pin;
                const MASK: u8 = 1 << $bit;
            }
        )*
    };
}

pins! {
    a0: Pa0 => (PORTA, DDRA, PINA, 0),
    a1: Pa1 => (PORTA, DDRA, PINA, 1),
    a2: Pa2 => (PORTA, DDRA, PINA, 2),
    a3: Pa3 => (PORTA, DDRA, PINA, 3),
    a4: Pa4 => (PORTA, DDRA, PINA, 4),
    a5: Pa5 => (PORTA, DDRA, PINA, 5),
    a6: Pa6 => (PORTA, DDRA, PINA, 6),
    a7: Pa7 => (PORTA, DDRA, PINA, 7),

    b0: Pb0 => (PORTB, DDRB, PINB, 0),
    b1: Pb1 => (PORTB, DDRB, PINB, 1),
    b2: Pb2 => (PORTB, DDRB, PINB, 2),
    b3: Pb3 => (PORTB, DDRB, PINB, 3),
    b4: Pb4 => (PORTB, DDRB, PINB, 4),
    b5: Pb5 => (PORTB, DDRB, PINB, 5),
    b6: Pb6 => (PORTB, DDRB, PINB, 6),
    b7: Pb7 => (PORTB, DDRB, PINB, 7),

    c0: Pc0 => (PORTC, DDRC, PINC, 0),
    c1: Pc1 => (PORTC, DDRC, PINC, 1),
    c2: Pc2 => (PORTC, DDRC, PINC, 2),
    c3: Pc3 => (PORTC, DDRC, PINC, 3),
    c4: Pc4 => (PORTC, DDRC, PINC, 4),
    c5: Pc5 => (PORTC, DDRC, PINC, 5),
    c6: Pc6 => (PORTC, DDRC, PINC, 6),
    c7: Pc7 => (PORTC, DDRC, PINC, 7),

    d0: Pd0 => (PORTD, DDRD, PIND, 0),
    d1: Pd1 => (PORTD, DDRD, PIND, 1),
    d2: Pd2 => (PORTD, DDRD, PIND, 2),
    d3: Pd3 => (PORTD, DDRD, PIND, 3),
    d4: Pd4 => (PORTD, DDRD, PIND, 4),
    d5: Pd5 => (PORTD, DDRD, PIND, 5),
    d6: Pd6 => (PORTD, DDRD, PIND, 6),
    d7: Pd7 => (PORTD, DDRD, PIND, 7),
}
