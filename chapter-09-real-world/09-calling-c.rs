// Chapter 9, Lesson 9: Calling C code (FFI)
// Run: cargo run --example ch09-09

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

// FFI ("foreign function interface") is like using an interpreter to talk
// to an older colleague who only speaks C. We declare what the C functions
// look like; the C standard library is linked in already.
extern "C" {
    fn abs(x: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn toupper(c: c_int) -> c_int;
}

// A safe Rust wrapper. Callers never write `unsafe` themselves.
fn c_strlen(text: &str) -> usize {
    let c_text = CString::new(text).expect("no zero bytes allowed");
    // SAFETY: c_text is a valid, zero-terminated string that lives
    // until the end of this function.
    unsafe { strlen(c_text.as_ptr()) }
}

fn shout(text: &str) -> String {
    text.bytes()
        // SAFETY: toupper accepts any value that fits in an unsigned char.
        .map(|b| unsafe { toupper(b as c_int) } as u8 as char)
        .collect()
}

fn main() {
    // Calling C is `unsafe`: Rust cannot check what the C code does.
    let n = unsafe { abs(-42) };
    println!("abs(-42) from C = {n}");
    println!("strlen(\"namaste\") from C = {}", c_strlen("namaste"));
    println!("toupper from C: {}", shout("rust and c"));

    // Going the other way: a C string back into Rust.
    let owned = CString::new("back to Rust").unwrap();
    let borrowed: &CStr = owned.as_c_str();
    println!("CStr to &str: {}", borrowed.to_str().unwrap());
}

// Rust can also export functions for C to call:
#[no_mangle]
pub extern "C" fn rust199_add(a: c_int, b: c_int) -> c_int {
    a + b
}
