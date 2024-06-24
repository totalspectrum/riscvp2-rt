//
// Simple runtime library for riscvp2
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

//
// (theoretically) useful CSR registers for the emulated RISC-V
// in practice these aren't very useful yet because asm! doesn't
// allow constants; need to look into this
//

pub const CNT_CSR:  u32 = 0xc00;
pub const CNT_CSRH: u32 = 0xc80;

pub const UART_CSR:    u32 = 0xbc0;
pub const WAITCYC_CSR: u32 = 0xbc1;
pub const DBGPRNT_CSR: u32 = 0xbc2;
pub const MILLIS_CSR:  u32 = 0xbc3;
pub const UART_STATUS_CSR: u32 = 0xbc4;

//
// these methods are basically the same as the standard P2 system
// functions in C <propeller2.h> or Spin2
//

// set a pin low
#[inline(always)]
pub fn pinl( p: u32 ) {
  unsafe {
    asm!(
      ".insn s CUSTOM_0, 2, x0, 0x000({z})",
        z = in(reg) p
      );
  }
}

// set a pin high
pub fn pinh( p: u32 ) {
  unsafe {
    asm!(
      ".insn s CUSTOM_0, 2, x0, 0x400({z})",
        z = in(reg) p
      );
  }
}

// toggle a pin
#[inline(always)]
pub fn pintoggle( p: u32 ) {
  unsafe {
    asm!(
      ".insn s CUSTOM_0, 2, x0, -0x400({z})",
        z = in(reg) p
      );
  }
}

// float a pin low
#[inline(always)]
pub fn pinf( p: u32 ) {
  unsafe {
    asm!(
      ".insn s CUSTOM_0, 3, x0, 0x000({z})",
        z = in(reg) p
      );
  }
}

// write data to smart pin mode register
#[inline(always)]
pub fn wrpin( p: u32, data: u32 ) {
  unsafe {
    asm!(
      ".insn s CUSTOM_0, 6, {v}, 0x000({z})",
        z = in(reg) p,
	v = in(reg) data
      );
  }
}

// write data to smart pin X register
#[inline(always)]
pub fn wxpin( p: u32, data: u32 ) {
  unsafe {
    asm!(
      ".insn s CUSTOM_0, 6, {v}, 0x400({z})",
        z = in(reg) p,
	v = in(reg) data
      );
  }
}

// write data to smart pin Y register
#[inline(always)]
pub fn wypin( p: u32, data: u32 ) {
  unsafe {
    asm!(
      ".insn s CUSTOM_0, 6, {v}, -0x800({z})",
        z = in(reg) p,
	v = in(reg) data
      );
  }
}

// read value from smart pin
#[inline(always)]
pub fn rdpin( p: u32 ) -> u32 {
  let mut result : u32;
  unsafe {
    asm!(
      ".insn i CUSTOM_0, 7, {v}, 0x400({z})",
        z = in(reg) p,
	v = out(reg) result
      );
  }
  result
}

// delay by some number of cycles
#[inline(always)]
pub fn waitx( n: u32 ) {
  unsafe {
    asm!(
      ".insn i CUSTOM_1, 1, x0, 31({zi})",
        zi = in(reg) n
      );
  }
}

// delay by some number of milliseconds
#[inline(always)]
pub fn waitms( ms: u32 ) {
  unsafe {
    // 0xbc3 is MILLIS_CSR
    asm!(
      "csrw 0xbc3, {rs}",
      rs = in(reg) ms
      );
  }
}
