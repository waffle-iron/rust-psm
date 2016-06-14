extern crate libc;

use std::os::unix::io::RawFd;
use std::ffi::CString;
use fileops::{open, close};
use errno::{errno, strerrer};

fn ipath_context_open(unit: isize) -> Option<libc::c_int> {
  let dev_path = match unit {
    u if unit >= 0 => format!("/dev/ipath{}", u),
    _ => format!("/dev/ipath")
  };

  let fd = open(dev_path, libc::O_RDWR);

  match fd {
    -1 => None,
    _ => unsafe {
      match libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) {
        -1 => None,
        _ => Some(fd)
      }
    }
  }
}

#[test]
#[ignore]
/// Requires /dev/ipath to be created by ib_qib, ignore for travis
// TODO: add a test for checking all available units
fn open_close_unit_zero() {
  let fd_maybe = ipath_context_open(0);
  match fd_maybe {
    None => panic!("errno: {}, errno str {}", errno(), strerrer(errno())),
    Some(fd) => {
      match close(fd) {
        -1 => panic!("errno: {}, errno str {}", errno(), strerrer(errno())),
        _ => ()
      }
    }
  }
}
