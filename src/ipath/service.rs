extern crate libc;

use std::ffi::CString;
use fileops::Fd;
use std::io::Error;
use std::error::Error as std_error;

fn ipath_context_open(unit: isize) -> Result<Fd, Error> {
  let dev_path = if unit >= 0 {
    format!("/dev/ipath{}", unit)
  } else {
    format!("/dev/ipath")
  };

  // XXX: Do we need ipath_wait_for_device? it literally just waits.

  // Try to get a Fd and try to set the CLOEXEC flag on it.
  let fd = try!(Fd::open(dev_path, libc::O_RDWR));
  try!(fd.try_set_flag(libc::FD_CLOEXEC));
  Ok(fd)
}

#[test]
// ignored because /dev/ipath to be created by ib_qib, travis doesn't have
// the proper hardware to do tests based on the driver.
#[ignore]
// TODO: add a test for checking all available units
fn open_close_unit_zero() {
  match ipath_context_open(0) {
    Err(e) => panic!(e.description().to_owned()),
    _ => ()
  }
}
