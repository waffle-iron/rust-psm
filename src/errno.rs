#![macro_use]

extern crate libc;

use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;
use libc::c_int;
use fileops::open;

pub fn strerror(errno: c_int) -> String {
  unsafe {
    CStr::from_ptr(libc::strerror(errno)).to_string_lossy().into_owned()
  }
}

pub fn errno() -> c_int {
  unsafe { *libc::__errno_location() }
}

macro_rules! dump_errno_str {
  () => (format!("errno: {}, errno str {}", errno(), strerror(errno())))
}

#[test]
/// Forcing EISDIR by setting O_WRONLY on /tmp
fn errno_e_is_dir() {
  let fd = open("/tmp", libc::O_WRONLY);
  assert_eq!(errno(), libc::EISDIR);
  assert_eq!(strerror(errno()), "Is a directory");
}
