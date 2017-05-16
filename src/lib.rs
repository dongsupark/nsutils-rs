extern crate procinfo;
extern crate unshare;
#[macro_use]
extern crate text_io;

mod nsutils;

pub use nsutils::{ListNs, NsCtx, StatNs, NamespaceFile};
pub use nsutils::{ns_str_to_const, ns_const_to_str, read_proc_to_statns, statns_to_nslist};
