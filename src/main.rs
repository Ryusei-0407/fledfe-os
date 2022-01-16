#![no_std]
#![no_main]
#![feature(core_intrinsics)]

use core::intrinsics;
use core::panic::PanicInfo;

struct VGACell {
    is_blinking: u1,
    background_color: u3,
    is_bright: u1,
    character_color: u3,
    character: u8,
}

#[repr(u8)]
enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    Gray = 7,
    White = 8,
    BrightBlue = 9,
    BrightGreen = 10,
    BrightCyan = 11,
    BrightRed = 12,
    BrightMagenta = 13,
    Yellow = 14,
    DarkGray = 15,
}

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        intrinsics::abort();
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut framebuffer = 0xb8000 as *mut u8;

    unsafe {
        framebuffer.offset(1).write_volatile(0x30); // *(framebuffer + 1) = 0x30;
    }

    loop {}
}
