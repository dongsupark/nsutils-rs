extern crate argparse;
extern crate nsutils;
extern crate procinfo;
extern crate unshare;


use argparse::{ArgumentParser, Collect};
use nsutils::{print_nslist, read_proc_to_statns, statns_to_nslist};
use std::path::Path;

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
