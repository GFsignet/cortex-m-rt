#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, exception, interrupt};

#[must_use] //~ ERROR this attribute is not allowed on a function controlled by cortex-m-rt
#[entry]
fn foo() -> ! {
    loop {}
}

#[must_use] //~ ERROR this attribute is not allowed on a function controlled by cortex-m-rt
#[exception]
fn SysTick() {}

#[allow(non_camel_case_types)]
enum interrupt {
    USART1,
}

#[must_use] //~ ERROR this attribute is not allowed on a function controlled by cortex-m-rt
#[interrupt]
fn USART1() {}
