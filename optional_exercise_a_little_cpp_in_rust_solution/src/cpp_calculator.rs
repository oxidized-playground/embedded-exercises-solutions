use super::calculator::Calculator;

#[repr(C)]
pub struct CppCalculator;

extern "C" {
    fn cpp_calculator_new() -> *mut std::ffi::c_void;
    fn cpp_calculator_delete(calc: *mut std::ffi::c_void);
    fn cpp_whothis(calc: *mut std::ffi::c_void);
    fn cpp_add(calc: *mut std::ffi::c_void, x: i16, y: i16) -> i16;
    fn cpp_subtract(calc: *mut std::ffi::c_void, x: i32, y: i32) -> i32;
    fn cpp_multiply(calc: *mut std::ffi::c_void, x: u32, y: u32) -> u32;
}

impl Calculator for CppCalculator {
    fn whothis(&self) {
        unsafe {
            let calc = cpp_calculator_new();
            cpp_whothis(calc);
            cpp_calculator_delete(calc);
        }
    }

    fn add(&self, x: i16, y: i16) -> i16 {
        unsafe {
            let calc = cpp_calculator_new();
            let result = cpp_add(calc, x, y);
            cpp_calculator_delete(calc);
            result
        }
    }

    fn subtract(&self, x: i32, y: i32) -> i32 {
        unsafe {
            let calc = cpp_calculator_new();
            let result = cpp_subtract(calc, x, y);
            cpp_calculator_delete(calc);
            result
        }
    }

    fn multiply(&self, x: u32, y: u32) -> u32 {
        unsafe {
            let calc = cpp_calculator_new();
            let result = cpp_multiply(calc, x, y);
            cpp_calculator_delete(calc);
            result
        }
    }
}