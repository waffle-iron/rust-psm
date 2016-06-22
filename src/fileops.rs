//! Contains operations related to the /dev/ipath character files.

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

  fn close(&self) -> c_int {
    unsafe { libc::close(self.0) }
  }

  pub fn try_set_flag(&self, flag: c_int ) -> Option<c_int> {
    match unsafe { libc::fcntl(self.0, libc::F_SETFD, flag) } {
      -1 => None,
      _ => Some(0)
    }
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
