#![no_std]

use avr::{
    delay_ms, delay_us,
};
use core::fmt;
use hal::{Level, SetLevel};

enum Command {
    ClearDisplay = 0x01,
    _ReturnHome = 0x02,
    _EntryModeSet = 0x04,
    _DisplayControl = 0x08,
    _CursorShift = 0x10,
    _FunctionSet = 0x20,
    _SetCGRamAddr = 0x40,
    _SetDDRamAddr = 0x80,
}

/// A 4-bit HD44780 LCD.
pub struct Lcd<D7, D6, D5, D4, E, Rs>
where
    D7: SetLevel,
    D6: SetLevel,
    D5: SetLevel,
    D4: SetLevel,
    E: SetLevel,
    Rs: SetLevel,
{
    pub d7: D7,
    pub d6: D6,
    pub d5: D5,
    pub d4: D4,
    pub e: E,
    pub rs: Rs,
}

impl<D7, D6, D5, D4, E, Rs> Lcd<D7, D6, D5, D4, E, Rs>
where
    D7: SetLevel,
    D6: SetLevel,
    D5: SetLevel,
    D4: SetLevel,
    E: SetLevel,
    Rs: SetLevel,
{
    #[inline(never)]
    pub fn init(&mut self) {
        self.rs.set_low();

        // 4-bit mode
        {
            delay_ms(50);

            self.send_nibble(0x03 << 4);
            delay_ms(5); // Wait for more than 4.1ms

            self.send_nibble(0x03 << 4);
            delay_us(150); // Wait for more than 100us

            self.send_nibble(0x03 << 4);
            delay_us(150); // Wait for more than 100us

            // Now we switch to 4-bit mode
            self.send_nibble(0x02 << 4);
        }

        // Finally, set # lines, font size
        self.command(0x28);
        self.command(0x08);
        self.command(0x01);
        self.command(0x06);
        self.command(0x0c);
    }

    #[inline(never)]
    pub fn write(&mut self, s: &str) {
        self.command(0x80);

        for b in s.bytes() {
            self.data(b);
        }
    }

    #[inline(never)]
    pub fn clear(&mut self) {
        self.command(Command::ClearDisplay as u8);
        // This command could take as long as 1.52ms to execute
        delay_ms(2);
    }

    // #[inline(never)]
    fn pulse_enable(&mut self) {
        self.e.set_high();
        delay_us(1);
        self.e.set_low();
        delay_us(50);
    }

    #[inline(never)]
    fn send_nibble(&mut self, data: u8) {
        self.d7.set_level(Level::from((data & 0x80) > 0));
        self.d6.set_level(Level::from((data & 0x40) > 0));
        self.d5.set_level(Level::from((data & 0x20) > 0));
        self.d4.set_level(Level::from((data & 0x10) > 0));

        self.pulse_enable();
    }

    #[inline(never)]
    fn send_byte(&mut self, data: u8) {
        self.send_nibble(data);
        self.send_nibble(data << 4);
    }

    #[inline(never)]
    fn command(&mut self, cmd: u8) {
        self.rs.set_low();
        self.send_byte(cmd);
        delay_us(500);
    }

    #[inline(never)]
    fn data(&mut self, data: u8) {
        self.rs.set_high();
        self.send_byte(data);
        self.rs.set_low();
        delay_us(500);
    }
}

impl<D7, D6, D5, D4, E, Rs> fmt::Write for Lcd<D7, D6, D5, D4, E, Rs>
where
    D7: SetLevel,
    D6: SetLevel,
    D5: SetLevel,
    D4: SetLevel,
    E: SetLevel,
    Rs: SetLevel,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}
