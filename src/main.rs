use std::mem;

use libc::{c_char, c_int, c_uint};
use std::ffi::CStr;

/// only used property in getifaddrs() in c is defined
#[repr(C)]
pub struct IfAddrs {
    ifa_next: *mut IfAddrs,
    ifa_name: *mut c_char,
    ifa_flags: c_uint,
    ifa_addr: *mut IfAddrs,
}

extern "C" {
    pub fn getifaddrs(ifap: *mut *mut IfAddrs) -> c_int;
}

fn main() {
    let mut ifap: *mut IfAddrs = unsafe { mem::zeroed() };

    unsafe {
        let n = getifaddrs(&mut ifap);
        if n == 0 {
            println!("{:?}", CStr::from_ptr((*ifap).ifa_name));
        }
    }
}
