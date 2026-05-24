#![no_std]

use hal::{Device, SetLevel, Transfer};

/// [ADXL345] 3-axis accelerometer.
///
/// [ADXL345]: https://www.analog.com/media/en/technical-documentation/data-sheets/adxl345.pdf
pub struct Adxl345<Cs: SetLevel>(Device<Cs>);

impl<Cs: SetLevel> Adxl345<Cs> {
    pub fn new(mut device: Device<Cs>, spi: &mut impl Transfer) -> Self {
        spi.transfer_batch(&mut device, &mut [0x2d, 0x08]);
        Self(device)
    }

    pub fn read_xyz_blocking(&mut self, spi: &mut impl Transfer) -> (i16, i16, i16) {
        let mut buf = [0xf2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        spi.transfer_batch(&mut self.0, &mut buf);
        let [_, x0, x1, y0, y1, z0, z1] = buf;

        let x = i16::from_le_bytes([x0, x1]);
        let y = i16::from_le_bytes([y0, y1]);
        let z = i16::from_le_bytes([z0, z1]);

        (x, y, z)
    }
}
