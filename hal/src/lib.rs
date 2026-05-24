#![no_std]

/// Digital input or output level.
pub enum Level {
    High,
    Low,
}

impl From<bool> for Level {
    fn from(value: bool) -> Self {
        match value {
            true => Self::High,
            false => Self::Low,
        }
    }
}

/// Set the output level of a pin.
pub trait SetLevel {
    /// Sets the output level to high.
    fn set_high(&mut self);

    /// Sets the output level to low.
    fn set_low(&mut self);

    /// Sets the output level acording to a [`Level`].
    fn set_level(&mut self, level: Level) {
        match level {
            Level::High => self.set_high(),
            Level::Low => self.set_low(),
        }
    }
}

/// Unsafely create an instance of this peripheral out of thin air.
pub trait Steal {
    unsafe fn steal() -> Self;
}

/// Delay with up to nanosecond precision.
pub trait Delay {
    fn delay_ns(&mut self, ns: u32);

    fn delay_us(&mut self, us: u32);

    fn delay_ms(&mut self, ms: u32);
}

/// Transfer bytes via SPI protocol.
pub trait Transfer {
    fn transfer_batch(&mut self, device: &mut Device<impl SetLevel>, batch: &mut [u8]);
}

/// SPI device.
pub struct Device<Cs: SetLevel>(pub Cs);

impl<Cs: SetLevel> Device<Cs> {
    pub fn new(mut cs: Cs) -> Self {
        cs.set_high();
        Self(cs)
    }
}
