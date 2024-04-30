use std::ffi::CStr;
use std::os::raw::c_char;

pub fn cstr_to_string(cstr: *const c_char) -> String {
    // Safety: We assume that the input pointer is valid and points to a null-terminated string.
    unsafe {
        // Create a CStr from the input pointer
        let c_str = CStr::from_ptr(cstr);

        // Convert the CStr to a Rust String
        match c_str.to_str() {
            Ok(s) => s.to_owned(), // If the conversion succeeds, return the owned String
            Err(_) => panic!("Failed to convert C string to Rust string"), // Handle conversion failure
        }
    }
}

pub fn c_string(s: &str) -> *mut c_char {
    // Convert the Rust string to a C string
    std::ffi::CString::new(s)
        .expect("Failed to convert Rust string to C string")
        .into_raw()
}
