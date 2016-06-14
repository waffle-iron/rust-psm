extern crate libc;
use std::ffi::CString;
use std::os::raw::c_char;
use libc::c_int;
use fileops::open;

pub fn strerrer(errno: c_int) -> String {
  unsafe {
    let c_err_str = libc::strerror(errno);
    CString::from_raw(c_err_str).into_string()
      .unwrap_or(String::from("Unknown errno"))
  }
}

pub fn errno() -> c_int {
  unsafe { *libc::__errno_location() }
}

#[test]
/// Forcing EISDIR by setting O_WRONLY on /tmp
fn errno_eisdir() {
  let fd = open("/tmp", libc::O_WRONLY);
  assert_eq!(errno(), libc::EISDIR);
}
