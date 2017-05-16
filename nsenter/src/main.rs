extern crate argparse;
extern crate nsutils;
extern crate libc;
extern crate procinfo;
extern crate unshare;

use argparse::{ArgumentParser, Collect, PushConst, Store};
use nsutils::NamespaceFile;
use nsutils::ns_const_to_str;

use std::io::{stderr, Write};
use std::os::unix::io::RawFd;
use unshare::Namespace;

unsafe fn do_setns(nstype: Namespace, fd: RawFd) -> i32 {
    let ret = libc::setns(fd, nstype as i32);
    if ret != 0 {
        writeln!(&mut stderr(),
                 "Error from setns on fd {} with nstype {}, ret = {}",
                 fd,
                 ns_const_to_str(&nstype),
                 ret)
                .ok();
    }

    ret
}

fn setup_ns() -> Vec<NamespaceFile> {
    vec![NamespaceFile {
             nstype: Namespace::User,
             name: "/proc/self/ns/user".to_string(),
             fd: -1,
         },
         NamespaceFile {
             nstype: Namespace::Ipc,
             name: "/proc/self/ns/ipc".to_string(),
             fd: -1,
         },
         NamespaceFile {
             nstype: Namespace::Uts,
             name: "/proc/self/ns/uts".to_string(),
             fd: -1,
         },
         NamespaceFile {
             nstype: Namespace::Net,
             name: "/proc/self/ns/net".to_string(),
             fd: -1,
         },
         NamespaceFile {
             nstype: Namespace::Pid,
             name: "/proc/self/ns/pid".to_string(),
             fd: -1,
         },
         NamespaceFile {
             nstype: Namespace::Mount,
             name: "/proc/self/ns/mnt".to_string(),
             fd: -1,
         }]
}

fn main() {
    let mut command = "".to_string();
    let mut args: Vec<String> = Vec::new();
    let mut namespaces: Vec<Namespace> = Vec::<Namespace>::new();

    {
        let mut ap = ArgumentParser::new();

        ap.set_description("enter a Linux namespace");
        ap.refer(&mut command)
            .add_argument("command", Store, "Command to run")
            .required();
        ap.refer(&mut args)
            .add_argument("arg", Collect, "Arguments for the command")
            .required();
        ap.refer(&mut namespaces)
            .add_option(&["--enter-ipc"],
                        PushConst(Namespace::Ipc),
                        "Enter IPC namespace")
            .add_option(&["--enter-pid"],
                        PushConst(Namespace::Pid),
                        "Enter PID namespace")
            .add_option(&["--enter-net"],
                        PushConst(Namespace::Net),
                        "Enter net namespace")
            .add_option(&["--enter-mount"],
                        PushConst(Namespace::Mount),
                        "Enter mount namespace")
            .add_option(&["--enter-uts"],
                        PushConst(Namespace::Uts),
                        "Enter UTS namespace")
            .add_option(&["--enter-user"],
                        PushConst(Namespace::User),
                        "Enter user namespace");
        ap.stop_on_first_argument(false);
        ap.parse_args_or_exit();
    }

    let mut cmd = unshare::Command::new(&command);
    cmd.args(&args[..]);

    let namespace_files = setup_ns();

    let mut nf_valid: Vec<NamespaceFile> = Vec::<NamespaceFile>::new();

    for nf in namespace_files {
        let nstype = nf.nstype;
        let mut nfmut = nf;
        if namespaces.contains(&nstype) {
            nfmut.open_file();
            nf_valid.push(nfmut);
        }
    }

    for nf in nf_valid {
        let fd = nf.fd;
        if fd < 0 {
            continue;
        }
        unsafe {
            let ret = do_setns(nf.nstype, fd);
            if ret != 0 {
                continue;
            }
        }
    }
}
