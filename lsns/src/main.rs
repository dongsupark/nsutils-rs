extern crate unshare;
extern crate argparse;

use unshare::Namespace;
use argparse::{ArgumentParser, Collect, ParseOption};

fn main() {
    let mut args: Vec<String> = Vec::new();
    let mut namespaces = Vec::<Namespace>::new();

    {
        let mut ap = ArgumentParser::new();

        ap.set_description("List all Linux namespaces");
        ap.refer(&mut args)
            .add_argument("arg", Collect, "Arguments for the command")
            .required();
        ap.stop_on_first_argument(false);
        ap.parse_args_or_exit();
    }
}
