//! Contains operations related to the /dev/ipath character files.
//! fcntl is not included because rust does not allow for the C-style
//! argument list (...) outside of ffi functions. fcntl is only used once
//! in the C version.

extern crate libc;

use std::ffi::CString;
use libc::c_int;

pub fn open<T: Into<String>>(path: T, mode: c_int) -> c_int {
  unsafe { libc::open(CString::new(path.into())
                      .unwrap_or(CString::new("").unwrap()).as_ptr(), mode) }
}

pub fn close(fd: c_int) -> c_int {
  unsafe { libc::close(fd) }
}

#[test]
// Check open/close on a file that should exist in most linux based OS.
fn open_close_devnull() -> () {
  let fd = open("/dev/null", libc::O_RDONLY);
  assert!(fd >= 0);
  let ret = close(fd);
  assert!(ret != -1);
}
