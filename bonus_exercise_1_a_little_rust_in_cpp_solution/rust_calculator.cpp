//
// Created by Sidney on 27/09/2023.
//

#include "rust_calculator.h"

// When using external libraries. You need to declare the functions as extern "C" to avoid name mangling.
// To use the function from the Rust library, you need to define the external interface in an extern "C" block.
// The extern "C" block tells the compiler to use the C calling convention for the functions inside the block.

// TODO: define the functions that are declared in the rust library

extern "C" {
    void rust_whothis();
    uint16_t rust_add(uint16_t a, uint16_t b);
    int32_t rust_subtract(int32_t a, int32_t b);
    uint32_t rust_multiply(uint32_t a, uint32_t b);
}

RustCalculator::RustCalculator() {}

void RustCalculator::whothis() {
   rust_whothis();
}

uint16_t RustCalculator::add(uint16_t x, uint16_t y) {
    return rust_add(x, y);
}

int32_t RustCalculator::subtract(int32_t x, int32_t y) {
    return rust_subtract(x, y);
}

uint32_t RustCalculator::multiply(uint32_t x, uint32_t y) {
    return rust_multiply(x, y);
}