#![no_std]

#[allow(unused_imports)]
use panic_halt; // When a panic occurs, simply stop the microcontroller
use stm32f3xx_hal::{delay::Delay, pac, prelude::*};

#[no_mangle]
pub extern "C" fn rust_function() -> i32 {
    calculate_delay() as i32
}

fn calculate_delay() -> u16 {
    1500
}
