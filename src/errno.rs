//! Provides an easier way to access the linux driver error functions
//! such as errno and strerror. This module is needed because many C functions
//! set serrno in the case of failures. Since PSM needs to use the /dev/ipath
//! character file, errno gets set anytime the ib_qib driver encounters a
//! problem.

#![macro_use]

extern crate libc;

use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;
use libc::c_int;
use fileops::Fd;

pub fn strerror(errno: c_int) -> String {
  unsafe {
    CStr::from_ptr(libc::strerror(errno)).to_string_lossy().into_owned()
  }
}

pub fn errno() -> c_int {
  unsafe { *libc::__errno_location() }
}

#[macro_export]
macro_rules! dump_errno_str {
  () => {{
    use $crate::errno;
    format!("errno: {}, errno str {}", errno(), strerror(errno()))
  }}
}

#[test]
// Forcing EISDIR by setting O_WRONLY on /tmp
fn errno_e_is_dir() {
  let fd = Fd::open("/tmp", libc::O_WRONLY);
  assert_eq!(errno(), libc::EISDIR);
  assert_eq!(strerror(errno()), "Is a directory");
}
