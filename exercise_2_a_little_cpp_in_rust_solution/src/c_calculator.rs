use super::calculator::Calculator;

pub struct CCalculator;

extern "C" {
    fn c_whothis();
    fn c_add(x: i16, y: i16) -> i16;
    fn c_subtract(x: i32, y: i32) -> i32;
    fn c_multiply(x: u32, y: u32) -> u32;
}

impl Calculator for CCalculator {
    fn whothis(&self) {
        unsafe {c_whothis()};
    }
    fn add(&self, x: i16, y: i16) -> i16 {
        unsafe {c_add(x, y)}
    }

    fn subtract(&self, x: i32, y: i32) -> i32 {
        unsafe {c_subtract(x, y)}
    }

    fn multiply(&self, x: u32, y: u32) -> u32 {
        unsafe {c_multiply(x, y)}
    }
}