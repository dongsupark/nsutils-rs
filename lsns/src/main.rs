extern crate argparse;
extern crate nsutils;
extern crate procinfo;
extern crate unshare;

use argparse::{ArgumentParser, Collect};
use nsutils::{ns_const_to_str, read_proc_to_statns, statns_to_nslist, ListNs};
use std::collections::HashMap;
use std::path::Path;

fn print_nslist(nslist: HashMap<u64, ListNs>) {
    let vec_result: Vec<(u64, ListNs)> = nslist.into_iter().collect();

    println!("{0: >10} {1: >6} {2: >6} {3: >6} {4: >6} {5:.66}",
             "NSID",
             "NSTYPE",
             "NPROC",
             "PID",
             "PPID",
             "COMMAND");
    for (nsid, listns) in vec_result {
        println!("{0: >10} {1: >6} {2: >6} {3: >6} {4: >6} {5:.66}",
               nsid,
               ns_const_to_str(&listns.nstype),
               listns.nproc,
               listns.pid,
               listns.ppid,
               listns.cmdline,
               );
    }
}

fn main() {
    let mut args: Vec<String> = Vec::new();

    {
        let mut ap = ArgumentParser::new();

        ap.set_description("List all Linux namespaces");
        ap.refer(&mut args)
            .add_argument("arg", Collect, "Arguments for the command")
            .required();
        ap.stop_on_first_argument(false);
        ap.parse_args_or_exit();
    }

    let result_svec = read_proc_to_statns(&Path::new("/proc")).unwrap();

    let result_nslist = statns_to_nslist(result_svec);

    print_nslist(result_nslist);
}
