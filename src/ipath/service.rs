extern crate libc;

use std::os::unix::io::RawFd;
use std::ffi::CString;
use fileops::{open, close};
use errno::*;

fn ipath_context_open(unit: isize) -> Option<libc::c_int> {
  let dev_path = if unit >= 0 {
    format!("/dev/ipath{}", unit)
  } else {
    format!("/dev/ipath")
  };

  // open and fcntl return -1 and set errno in the case of an error
  // the fd here is the result, but we need to check fcntl's result
  let fd = open(dev_path, libc::O_RDWR);
  // We can get away w/o checking open, because fcntl on fd -1 does nothing
  if unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) } >= 0 {
    Some(fd)
  } else {
    None
  }
}

#[test]
// ignored because /dev/ipath to be created by ib_qib, travis doesn't have
// the proper hardware to do tests based on the driver.
#[ignore]
// TODO: add a test for checking all available units
fn open_close_unit_zero() {
  let fd_maybe = ipath_context_open(0);
  match fd_maybe {
    None => panic!(dump_errno_str!()),
    Some(fd) => {
      match close(fd) {
        -1 => panic!(dump_errno_str!()),
        _ => ()
      }
    }
  }
}
