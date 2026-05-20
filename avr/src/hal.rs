pub enum Level {
    High,
    Low,
}

pub trait SetLevel {
    fn set_high(&mut self);

    fn set_low(&mut self);

    fn set_level(&mut self, level: Level) {
        match level {
            Level::High => self.set_high(),
            Level::Low => self.set_low(),
        }
    }
}

pub trait Steal {
    unsafe fn steal() -> Self;
}
