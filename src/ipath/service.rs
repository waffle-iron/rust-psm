extern crate libc;

use std::ffi::CString;
use fileops::Fd;
use errno::*;
use std::os::unix::io::AsRawFd;

fn ipath_context_open(unit: isize) -> Option<Fd> {
  let dev_path = if unit >= 0 {
    format!("/dev/ipath{}", unit)
  } else {
    format!("/dev/ipath")
  };

  // XXX: Do we need ipath_wait_for_device? it literally just waits.

  // open and fcntl return -1 and set errno in the case of an error
  // the fd here is the result, but we need to check fcntl's result
  let fd_maybe = Fd::open(dev_path, libc::O_RDWR);
  match fd_maybe {
    Some(fd) => {
      unsafe {
        if libc::fcntl(fd.as_raw_fd(), libc::F_SETFD, libc::FD_CLOEXEC) >= 0 {
          Some(fd)
        } else {
          None
        }
      }
    },
    None => None
  }
}

#[test]
// ignored because /dev/ipath to be created by ib_qib, travis doesn't have
// the proper hardware to do tests based on the driver.
#[ignore]
// TODO: add a test for checking all available units
fn open_close_unit_zero() {
  let fd_maybe = ipath_context_open(0);
  if fd_maybe.is_none() {
    panic!(dump_errno_str!());
  }
}
