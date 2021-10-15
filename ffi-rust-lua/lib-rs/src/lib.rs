use std::ffi::CString;
use std::mem::forget;
use std::os::raw::c_char;

#[no_mangle]
pub extern fn say_hello() -> *const c_char {
    let string = CString::new("Hello").unwrap();
    let pointer = string.as_ptr();
    forget(string);
    pointer
}
