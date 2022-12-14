#![no_std]
#![no_main]

use core::arch::global_asm;
use core::ptr;

// mod panic;

global_asm!(include_str!("startup_riscv64.s"));
// global_asm!(include_str!("startup_aarch64.s"));

#[no_mangle]
pub extern "C" fn c_entry() {
    // const UART0: *mut u8 = 0x0900_0000 as *mut u8; // aarch64
    const UART0: *mut u8 = 0x1000_0000 as *mut u8; // riscv64
    // let welcome_str = b"Hello Rust from OpenEmbedded in Aarch64!\n";
    let welcome_str = b"Hello Rust from OpenEmbedded in RISCV64!\n";
    for c in welcome_str {
        unsafe {
            ptr::write_volatile(UART0, *c);
        }
    }
}

use core::panic::PanicInfo;

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    loop {}
}
