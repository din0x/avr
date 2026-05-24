use core::marker::PhantomData;

use hal::Steal;

use crate::state::Uninit;

/// Normal timer mode.
pub struct Normal;

/// TC0 peripheral.
pub struct Timer0<S>(PhantomData<S>);

impl Steal for Timer0<Uninit> {
    unsafe fn steal() -> Self {
        Self(PhantomData)
    }
}

impl Timer0<Uninit> {
    pub fn into_normal(self) -> Timer0<Normal> {
        unsafe {
            raw::init();
        }

        Timer0(PhantomData)
    }
}

impl Timer0<Normal> {
    pub fn set_overflow_handler(&mut self, f: fn()) {
        _ = f();
        todo!()
    }

    pub fn set_compare_handler(&mut self, f: fn()) {
        _ = f();
        todo!()
    }
}

mod raw {
    pub unsafe fn init() {
        todo!()
    }
}
