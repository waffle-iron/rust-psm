//! Datatypes and functions related to Ep (endpoints) aka psm_ep_t,
//! (for now) EpAddr aka psm_epaddr_t, and Epid aka psm_epid_t.
extern crate uuid;

use error::Error;
use self::consts::*;
use uuid::Uuid;

pub mod consts;
#[macro_use]
pub mod macros;

// TODO: determine more appropriate types instead of passing -1/NULL, psm.h:449
// TODO: when rust supports conditional compilation add PSM_VERNO conditional fields
pub struct Ep {
  // TOOD: lots of things to add
  epid: Epid
}

pub struct EpOpts {
  timeout: u64,
  unit: isize,
  affinity: isize,
  shm_mbytes: isize,
  num_send_buffers: isize,
  network_pkey: u64,
  port: isize,
  ib_out_sl: isize,
  ib_service_id: u64,
  path_res_type: PathResType,
  num_send_descriptors: isize,
  ep_imm_size: isize
}

enum PathResType {
  NONE,
  OPP,
  UMAD
}

enum PtlAddr {
  // TODO: add ptl_epaddr type
  ptladdr_u3 ([u32; 2]),
  ptladdr_u64 (u64),
  ptladdr_data ([u8; 0])
}

pub struct Epaddr <'a> {
  // *ptl: ptl          TODO: ptl is a reference to the parent obj
  //ptlctrl: Ptl_ctl,
  epid: Epid,
  ep: Ep,
  // void *usr_ep_ctxt  TODO: this is a raw pointer, find what to do
  // TODO: add egrlong/data things
  ptl_addr: PtlAddr,
  mctxt_gihdi: [u64; IPATH_MAX_UNIT],
  mctxt_epid: [Epid; IPATH_MAX_UNIT],
  mctxt_epcount: usize,
  mctxt_nsconn: usize,
  mctxt_send_seqnum: u16,
  mctxt_recv_seqnum: u16,
  /* TODO: consider http://rustbyexample.com/enum.html to make this a linked list,
   * depending on how the C version works.
   */
  mctxt_current: Option<&'a Epaddr<'a>>,
  // outoforder_q: Mqsq TODO: make Mqsq type
  outoforder_c: usize,

  // Linked list of Epaddr for multi-context
  mctxt_master: Option<&'a Epaddr<'a>>,
  mctxt_prev: Option<&'a Epaddr<'a>>,
  mctxt_next: Option<&'a Epaddr<'a>>
}


impl Ep {
  // TODO: add psm_ep, add new
  pub fn open<'a>(job_key: Uuid, ep_opts: EpOpts) -> Result<(Ep, &'a Epaddr<'a>), Error> {
    return Err(Error::UnknownError)
  }

  pub fn close(ep: Ep, mode: isize, timeout: u64) -> Error {
    return Error::UnknownError
  }

  pub fn connect<'a>(ep: Ep, epids: Box<Vec<Epid>>, epid_masks: &Vec<isize>, timeout: i64) -> Result<Box<Vec<&'a Epaddr<'a>>>, Box<Vec<Error>>> {
    return Err(Box::new(vec!()))
  }
}

impl EpOpts {
  fn new() -> EpOpts {
    EpOpts {
      timeout: 1, unit: 1, affinity: 1, shm_mbytes: 1,
      num_send_buffers: 1, network_pkey: 1, port: 1,
      ib_out_sl: 1, ib_service_id: 1, path_res_type: PathResType::UMAD,
      num_send_descriptors: 1, ep_imm_size: 1
    }
  }
}

impl <'a>Epaddr <'a> {
  // TODO: figure out what fields are commonly used when epaddr is constructed
}
/* TODO: see if we can make this into a struct so we can real getters/setters that dont have the
 * words get/set
 */

pub type Epid = u64;

pub fn get_nid(epid: Epid) -> u64 {
  1
}

pub fn get_context(epid: Epid) -> u64 {
  1
}

pub fn get_port(epid: Epid) -> u64 {
  1
}
