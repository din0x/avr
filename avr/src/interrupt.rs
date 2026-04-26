use crate::registers::SREG;
use core::{cell::UnsafeCell, marker::PhantomData, ptr::read_volatile};

#[inline(always)]
pub fn critical_section(f: impl FnOnce(CriticalSection)) {
    let were_enabled = disable_interrupts();

    f(CriticalSection(PhantomData));

    if were_enabled {
        unsafe {
            enable_interrupts();
        }
    }
}

pub unsafe fn enable_interrupts() {
    unsafe {
        core::arch::asm!("sei");
    }
}

pub fn disable_interrupts() -> bool {
    let were_enabled = interrupts_enabled();

    unsafe {
        core::arch::asm!("cli");
    }

    were_enabled
}

fn interrupts_enabled() -> bool {
    unsafe { read_volatile(SREG) & (1 << 7) != 0 }
}

pub struct CriticalSection<'a>(PhantomData<&'a mut ()>);

// Mutex based on critical section.
pub struct Mutex<T> {
    cell: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            cell: UnsafeCell::new(value),
        }
    }

    pub fn borrow<'cs>(&'cs self, cs: CriticalSection<'cs>) -> &'cs T {
        _ = cs;
        unsafe { self.cell.as_ref_unchecked() }
    }
}

unsafe impl<T> Sync for Mutex<T> {}
