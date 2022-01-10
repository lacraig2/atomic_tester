use std::arch::asm;

enum TEST_MODE {
    xchg,
    inc,
}

const tmode: TEST_MODE = TEST_MODE::inc;

fn main() {
    let mut x: u64 = 0;
    println!(
        "x contains {} initially is a pointer at {:x}",
        x,
        std::ptr::addr_of!(x) as u64
    );
    for i in 0..10 {
        match tmode {
            TEST_MODE::xchg => unsafe {
                asm!("xchg {0}, {1}", 
                    inout(reg) x,
                    in(reg) 0x13371337+i);
            },
            TEST_MODE::inc => unsafe {
                asm!("inc dword ptr [{0}]", in(reg) &x);
            },
        }
        println!("x contains {:x} now", x);
    }
}
