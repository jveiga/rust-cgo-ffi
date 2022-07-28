use once_cell::sync::Lazy;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{atomic::AtomicU8, Arc, RwLock};

static ACTIVE: AtomicU8 = AtomicU8::new(0);

#[repr(C)]
pub struct bla{
    a: libc::c_uint,
}


// function calculates fibonacci
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

#[no_mangle]
pub unsafe extern "C" fn fib(n: libc::c_int) -> *const libc::c_uint {
    let n = n as u32;
    let result = fibonacci(n);
    let result = result as *const libc::c_uint;
    result
}

#[no_mangle]
pub unsafe extern "C" fn inc(b: *mut bla) {
    (*b).a+=1;
}

#[no_mangle]
pub unsafe extern "C" fn call() -> *mut bla{
    let mut b = Box::new(bla{a: 1});
    let p = b.as_mut() as _ ;
    Box::leak(b);
    p
}

// #[no_mangle]
// pub extern "C" fn hello(name: *const libc::c_char) {
//     let buf = unsafe { CStr::from_ptr(name).to_bytes() };
// }

#[no_mangle]
pub extern "C" fn print() {
    // println!("{:?}", STUFF.clone().read().unwrap());
}
