mod libc_errno;

use std::ffi::CString;
use libc::{open, read, O_RDONLY, c_int, c_void};
use libc_errno::{errno, Errno};

fn open_file(path: &str) -> Result<c_int, Errno> {
    let c_path = CString::new(path).expect("CString::new failed");
    let fd = unsafe { open(c_path.as_ptr(), O_RDONLY) };
    if fd == -1 {
        Err(Errno(errno()))
    } else {
        Ok(fd)
    }
}

fn read_file(fd: c_int, buffer: &mut [u8]) -> Result<usize, Errno> {
    let result = unsafe { read(fd, buffer.as_mut_ptr() as *mut c_void, buffer.len()) };
    if result == -1 {
        Err(Errno(errno()))
    } else {
        Ok(result as usize)
    }
}

fn main() {
    let path = "test.txt";
    match open_file(path) {
        Ok(fd) => {
            let mut buffer = [0; 1024];
            match read_file(fd, &mut buffer) {
                Ok(bytes_read) => {
                    println!("Read {} bytes from file.", bytes_read);
                }
                Err(e) => {
                    eprintln!("Error reading file: {}", e.0);
                }
            }
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e.0);
        }
    }
    println!("Hello, PolyYieldSecure!");
}
