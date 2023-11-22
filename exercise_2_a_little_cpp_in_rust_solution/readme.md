# How to compile and run me

This exercise compiles the C++ library automatically by compiling the Rust project and put's the library in the correct spot by itself.

- cargo run
- it's that easy

In this example you will also have to implement some `extern "C" {}` blocks to forward declare the functions. Make sure to not forget to call the C functions inside a `unsafe {}` block
