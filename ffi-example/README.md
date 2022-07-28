# ffi-example

## Notes

* exported functions need to be public and `extern "C"`
* structs need `#[repr(C)]` (Compiler can reorder fields with default `#[repr(Rust)]` and padding (when necessary).
* to share stuff across the divide, use Box and Box::forget.
* use std::lazy::Lazy if something needs to be static.
* crate needs `staticlib` for static linking and `cdylib` for dynamic linking.
* header file can be generated with `cbindgen` through cli or `build.rs`.
