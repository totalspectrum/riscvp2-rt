//
// Simple P2 LED blinker
// Written by Eric R. Smith <eric.smith@collabora.com>
// Copyright 2024 Collabora Ltd.
// SPDX-License-Identifier: MIT
//

#![no_std]

use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
  loop { }
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
  extern "Rust" {
      fn main() -> !;
  }
  main()
}

pub fn pintoggle( p: u32 ) {
  unsafe {
    asm!(
      ".insn s CUSTOM_0, 2, x0, -0x400({z})",
        z = in(reg) p
      );
  }
}

pub fn delay_cycles( n: u32 ) {
  unsafe {
    asm!(
      ".insn i CUSTOM_1, 1, x0, 31({zi})",
        zi = in(reg) n
      );
  }
}
