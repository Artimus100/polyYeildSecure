

use libc;

pub fn errno() -> i32 {
    unsafe { *libc::__errno_location() }
}

pub fn set_errno(e: Errno) {
    unsafe { *libc::__errno_location() = e.0 }
}

pub struct Errno(pub i32);
