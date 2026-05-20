use core::arch::asm;

const F_CPU: u32 = 1_000_000;

#[inline(never)]
fn delay(count: u32) {
    // Our asm busy-wait takes a 16 bit word as an argument,
    // so the max number of loops is 2^16
    let outer_count = count >> 16 & 0xffff;
    let last_count = ((count & 0xffff) + 1) as u16;

    for _ in 0..outer_count {
        unsafe {
            asm!(
                "1:",
                "sbiw {0}, 1",
                "brne 1b",
                in(reg_iw) 0u16,
            )
        }
    }

    unsafe {
        asm!(
            "1:",
            "sbiw {0}, 1",
            "brne 1b",
            in(reg_iw) last_count,
        )
    }
}

/// Pauses execution for *at least* `ms` milliseconds.
#[inline(never)]
pub fn delay_ms(ms: u32) {
    let us = ms * 1000;
    delay_us(us);
}

/// Pauses execution for *at least* `us` microseconds.
#[inline(never)]
pub fn delay_us(us: u32) {
    let ps_lp = const { (1_000_000_000 * 4) / F_CPU };

    let ps = us * 1_000;
    let loops = ps / ps_lp;
    delay(loops);
}
