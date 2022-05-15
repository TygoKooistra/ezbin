# About
Ezbin is a tool to generate binary files.
Right now it supports the basic data types integers, floats, and strings.


# Usage
There are examples in the examples folder.

## CLI
Parse the content of input.txt and write it to output.bin
* ezbin input.txt -o output.bin
* ezbin level.ezbin -o map.bin

Display usage:
* ezbin

## Language
Write 3 (32 bit) integers:
* 32 33 34
* 32i 33i 34i
* 32i32 33i32 34i32

Write 2 floats:
* 35f 36f
* 35.0f32 36.0f32

Write 2 doubles:
* 37f64 38.0f64

Write a string:
* "Hello World!\n"
* "Hello World!\n"UTF8

Write a c-string:
* "Hello World!\n" 0u8

Set the endianness (default is big):
* [ENDIAN LITTLE]


# Build
download the project
or use the git CLI:
* git clone https://github.com/TygoKooistra/ezbin

build using cargo:
* cargo build -r

the program should be in
* target/release/ezbin
or
* target/release/ezbin.exe

to install on Linux, copy the output file to /bin (or anywhere else in PATH):
* sudo cp ./target/release/ezbin /bin
