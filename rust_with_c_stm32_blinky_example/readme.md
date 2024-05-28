# Readme!

This example is created for a stm32f302R8 (Nucleo). https://www.st.com/en/evaluation-tools/nucleo-f302r8.html
You are free to edit this example to make it work on any STM32, as long as you
pay attention to a couple key points explained in this readme.

# Installation
Install the following:
- STM32CubeMX : https://www.st.com/content/st_com/en/stm32cubemx.html (You will need an STM account)
- GNU ARM Toolchain : https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads (Download the arm-gnu-toolchain-xxxx-mingw-w64-i686-arm-none-eabi.exe if you have windows for example)
- CMake : https://cmake.org/download/
- Ninja : https://github.com/ninja-build/ninja/releases

If you want to also flash and run the project
- OpenOCD : https://github.com/xpack-dev-tools/openocd-xpack/releases

# Setup
## Generate stm32 code
The code for the project should be generated before any building can occur.
- Open the blinky.ioc file in STM32CubeMX and press "generate code". No need to change anything here
- This will also regenerate the CMakeLists.txt file. We need to add some lines to it. 
  - On line number 60, add```set(RUST_LIB_DIR ${CMAKE_SOURCE_DIR}/rust_lib/target/thumbv7em-none-eabi/debug)
    link_directories(${RUST_LIB_DIR})```
  - On the end of the file, add ```target_link_libraries(${PROJECT_NAME}.elf ${RUST_LIB_DIR}/librust_lib.a)```

## Build the rust library
Everything is preconfigured, but you probably need to install the target on your system

- cd into rust_lib
- `rustup update`
- `rustup target install thumbv7em-none-eabi`
- `rustup target add thumbv7em-none-eabi`
- `cargo build`

Now there should be a target/thumbv7em-none-eabi/debug with in it a rust_lib.a file

## Build the project
As it is a CMake project, any editor should work, or use the following steps.

### Building
- Go to the rust_stm32 folder
- `mkdir build`
- `cd build`
- `cmake "-DCMAKE_MAKE_PROGRAM=C:\ninja-win\ninja.exe" -G Ninja ..`
- `cmake --build .`

### Flashing
- `C:\Users\XXX\AppData\Roaming\xPacks\...\openocd.exe -s C:\Users\XXX\AppData\Roaming\xPacks\...\openocd\scripts
  -f stunning-octo\rust_stm32\st_nucleo_f3.cfg -c "tcl_port disabled" -c "gdb_port disabled" -c "tcl_port disabled" 
-c "program \"stunning-octo/rust_stm32/cmake-build-debug/blinky.elf\"" -c reset -c shutdown`
- Add your own paths to the openOCD location
- make sure the path to the .elf is absolute or you're positioned correctly relative
- If you're on powershell, the \" escapes after -c "progam " don't work. Use ` instead
- Change the .cfg file if you're using a different STM
- Hail mary 

### CLion
Opening this project folder in CLion should configure build and deploy automatically, See the sources at the bottom

- build the project using the UI
- flash onto the stm32 using the UI

This project should be able to be debugged. Running the debugger should also step right into the rust library


## Other STM?
change it around!

The thing you need to watch for is the target. For the STM32F3 it's thumbv7em-none-eabi (or thumbv7em-none-eabihf for units with FPU)
Look up your STM on the internet and find what processor it's using and what the memory table looks like (including memory sizes)

First of all you need to replace the .ioc file with a new one. 
- Open up STM32CubeMX
- click on "home"
- click on "Start my project from ST board" Access to board selector
- search for your board (in the case of the original board for this project it's NUCLEO-F302R8)
- double click the result that you're looking for in the right bottom panel
- select yes on the question to initialize everything default
- In the project manager tab, think of a nice name
- set the toolchain/IDE to STM32CubeIDE
- Set the directory some other folder than this project folder
- click "generate code"
- Copy the .ioc file into this projects folder
- reopen the .ioc file and click generate code again
  - This procedure makes sure that the first generate code operation does not overwrite the whole project folder
- This project came with a st_nucleo_f3.cfg. This is used when flashing. If you are using another STM, for example an STM32f0
  then you need to copy the corresponding .cfg (st_nucleo_f0.cfg) from your OpenOCD install folder /\openocd\scripts\target\

! Remember to edit the CMakeLists.txt file as described above !
! Also make sure that you edit the file path according to the new target structure !

Now we can setup the rust library to compile for the right target

- open the Cargo.toml file and adjust line number 24 to suit your target
  - features = ["stm32f302x8", "rt"]
- Open the memory.x file and adjust the FLASH and RAM origin and size to suit your target
- Open the .cargo/config file and change the target to yours
  - target = "thumbv7em-none-eabi"
- In src/lib.rs there is a use for the STM HAL library. Change that to your needs

Now you should be able to build this project for your STM


Sources:
https://medium.com/@AlexanderObregon/integrating-rust-into-existing-c-c-projects-e0810dbddded
https://jonathanklimt.de/electronics/programming/embedded-rust/rust-on-stm32-2/

https://www.jetbrains.com/help/clion/embedded-development.html#prepare-tools




