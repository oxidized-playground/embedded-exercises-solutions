#[no_mangle]
pub extern "C" fn rust_whothis() {
    println!("Hi I'm a rust calculator!");
}

#[no_mangle]
pub extern "C" fn rust_add(x: u16, y: u16) -> u16 {
    x + y
}

#[no_mangle]
pub extern "C" fn rust_subtract(x: i32, y: i32) -> i32 {
    (x - y) as i32
}

#[no_mangle]
pub extern "C" fn rust_multiply(x: i32, y: i32) -> i32 {
    x * y * 2
}
