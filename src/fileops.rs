//! Contains operations related to the /dev/ipath character files.
//! fcntl is not included because rust does not allow for the C-style
//! argument list (...) outside of ffi functions. fcntl is only used once
//! in the C version.

extern crate libc;

use std::ffi::CString;
use libc::c_int;
use std::os::unix::io::{RawFd, AsRawFd};
use std::ops::Drop;

pub struct Fd(RawFd);

impl Fd {
  pub fn open<T: Into<String>>(path: T, mode: c_int) -> Option<Fd> {
    let fd = unsafe { libc::open(CString::new(path.into())
                .unwrap_or(CString::new("").unwrap()).as_ptr(), mode) };
    match fd {
      -1 => None,
      _ => Some(Fd(fd))
    }
  }

  fn close(&mut self) -> c_int {
    unsafe { libc::close(self.0) }
  }
}

impl AsRawFd for Fd {
  fn as_raw_fd(&self) -> RawFd {
    self.0
  }
}

impl Drop for Fd {
  // XXX: Do we need to check result of close?
  fn drop(&mut self) {
    if self.0 != -1 {
      self.close();
    }
  }
}

#[test]
// Check open/close on a file that should exist in most linux based OS.
fn open_close_devnull() -> () {
  let fd = Fd::open("/dev/null", libc::O_RDONLY);
  assert!(fd.is_some());
}
