//! Allows for reading of ib_qib attributes from sysfs files.
//! Examples include number of free contexts and InfiniBand GIDS.

use std::io::{Error, Read};
use std::fs::File;

static SYSFS_PATH: &'static str = "/sys/class/infiniband/qib";

/// Reads attributes from /sys/class/infiniband/qib[0-9]/ports/[0-1]/*
pub fn read_port_attr(unit: u32, port: u32,
                      attr: &'static str) -> Result<String, Error> {
  let path = format!("{}{}/ports/{}/{}", SYSFS_PATH, unit, port, attr);
  let mut f = try!(File::open(path));
  let mut port_val = String::new();
  try!(f.read_to_string(&mut port_val));
  Ok(port_val.replace("\n",""))
}

/// Reads attributes from /sys/class/infiniband/qib[0-9]/*
pub fn read_unit_attr(unit: u32, attr: &'static str)-> Result<String, Error> {
  let path = format!("{}{}/{}", SYSFS_PATH, unit, attr);
  let mut f = try!(File::open(path));
  let mut unit_val = String::new();
  try!(f.read_to_string(&mut unit_val));
  Ok(unit_val.replace("\n",""))
}

// Testing file: /sys/class/infiniband/qib0/hca_type
#[test]
// Requires: ib_qib to be loaded with a valid adapter
#[ignore]
fn check_unit_hca_type() {
  use std::error::Error as std_error;
  match read_unit_attr(0, "hca_type") {
    Ok(hca_type) => {
      if !hca_type.contains("InfiniPath_QLE") {
        panic!("Got invalid hca type {}, expected InfiniPath_QLE*", hca_type);
      }
    },
    Err(e) => panic!(e.description().to_owned())
  }
}

// Testing file: /sys/class/infiniband/qib0/ports/1/link_layer
#[test]
// Requires: ib_qib to be loaded with a valid adapter
#[ignore]
fn check_port_link_layer() {
  use std::error::Error as std_error;
  match read_port_attr(0, 1, "link_layer") {
    Ok(link_layer) => {
      if link_layer != "InfiniBand" {
        panic!("Got invalid link_layer {}, expected InfiniBand", link_layer);
      }
    },
    Err(e) => panic!(e.description().to_owned())
  }
}
