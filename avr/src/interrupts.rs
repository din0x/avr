//! AVR interrupt management.

use crate::registers::SREG;
use core::{cell::UnsafeCell, marker::PhantomData, ptr::read_volatile};

#[inline(always)]
pub fn critical_section(f: impl FnOnce(CriticalSection)) {
    let were_enabled = are_enabled();
    disable();

    f(CriticalSection(PhantomData));

    if were_enabled {
        unsafe {
            enable();
        }
    }
}

/// Enables interrupts.
pub unsafe fn enable() {
    unsafe {
        core::arch::asm!("sei");
    }
}

/// Disables interrupts.
pub fn disable() {
    unsafe {
        core::arch::asm!("cli");
    }
}

/// Returns `true` if interrupts are enabled.
pub fn are_enabled() -> bool {
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
