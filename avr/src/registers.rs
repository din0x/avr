macro_rules! registers {
    ($($name:ident = $addr:literal),* $(,)?) => {
        $(
            #[allow(unused)]
            pub const $name: *mut u8 = $addr as _;
        )*
    };
}

registers! {
    SREG   = 0x5F,
    SPH    = 0x5E,
    SPL    = 0x5D,
    MCUCR  = 0x55,
    MCUCSR = 0x54,
    GICR   = 0x5B,
    GIFR   = 0x5A,
    SPMCR  = 0x57,

    PINA   = 0x39,
    DDRA   = 0x3A,
    PORTA  = 0x3B,

    PINB   = 0x36,
    DDRB   = 0x37,
    PORTB  = 0x38,

    PINC   = 0x33,
    DDRC   = 0x34,
    PORTC  = 0x35,

    PIND   = 0x30,
    DDRD   = 0x31,
    PORTD  = 0x32,

    TCCR0  = 0x53,
    TCNT0  = 0x52,
    OCR0   = 0x5C,
    TIMSK  = 0x59,
    TIFR   = 0x58,

    TCCR1A = 0x4F,
    TCCR1B = 0x4E,
    TCNT1H = 0x4D,
    TCNT1L = 0x4C,
    OCR1AH = 0x4B,
    OCR1AL = 0x4A,
    OCR1BH = 0x49,
    OCR1BL = 0x48,
    ICR1H  = 0x47,
    ICR1L  = 0x46,

    TCCR2  = 0x45,
    TCNT2  = 0x44,
    OCR2   = 0x43,
    ASSR   = 0x42,

    ADMUX  = 0x27,
    ADCSRA = 0x26,
    ADCH   = 0x25,
    ADCL   = 0x24,

    UDR    = 0x2C,
    UCSRA  = 0x2B,
    UCSRB  = 0x2A,
    UCSRC  = 0x40, // special shared address
    UBRRH  = 0x40,
    UBRRL  = 0x29,

    SPCR   = 0x2D,
    SPSR   = 0x2E,
    SPDR   = 0x2F,

    TWBR   = 0x20,
    TWCR   = 0x36,
    TWSR   = 0x21,
    TWDR   = 0x23,
    TWAR   = 0x22,

    EEARH  = 0x3F,
    EEARL  = 0x3E,
    EEDR   = 0x3D,
    EECR   = 0x3C,

    ACSR   = 0x28,
}
