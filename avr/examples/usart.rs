#![no_std]

use avr::Peripherals;
use hal::Steal;

#[unsafe(no_mangle)]
fn main() {
    let pd = unsafe { Peripherals::steal() };
    let mut usart = pd.usart.into_init(9600, pd.d.0, pd.d.1);

    loop {
        let byte = usart.read_blocking();
        usart.write_blocking(byte);
    }
}
