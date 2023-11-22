# How to compile and run me

Prerequisite: Have CMake installed

- mkdir build
- cd build
- cmake ..
- cmake --build .
- Note: This will automatically build the Rust library already. 


- If you are working on Windows, the a_little_rust_in_cpp executable will be located in the Debug folder. If you are working on Linux, the a_little_rust_in_cpp executable will be inside a Debug folder and you have to copy the rust_calculator.lib into the Debug folder.
- If you are working on Linux, chances are that the library (rust_calculator.a) and the executable are in the same folder already


- Now run the executable

You will see that the output contains 0's for the calculations, which is not right. Try and fix the code using
`extern "C" {}` blocks to forward declare the library functions. Also make sure to keep the Rust compiler from mangling the Rust functions.