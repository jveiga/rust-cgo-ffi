# Rust C-FFI

# Commands

## In Rust folder

* cargo build --release
* cbindgen --crate ffi-example --lang=c  --output=call-ffi/lib/ffi-example.h 
* cp target/release/libffi_example.a call-ffi/lib/libffi_example.a 

## In Go folder

* go run main.go
