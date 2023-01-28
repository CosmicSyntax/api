use std::arch::asm;

pub fn add() -> u32 {
    let left: u32;
    let _right: u32;
    unsafe {
        asm!(
            "mov {0:w}, 2147483647",
            "mov {1:w}, {0:w}",
            "add {0:w}, {0:w}, 1",
            out(reg) left,
            out(reg) _right,
        );
    }
    left
}
