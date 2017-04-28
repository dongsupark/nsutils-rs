use procinfo::pid;
use procinfo::pid::Stat;
use std::collections::HashMap;
use std::fs::{self, DirEntry, File};
use std::io::{Read, Result};
use std::path::{Path, PathBuf};
use unshare::Namespace;

pub struct NsCtx {
    nsid: u64,
    nstype: Namespace,
}

pub struct StatNs {
    cmdline: String,
    stat: Stat,
    nses: Vec<NsCtx>,
}

impl IntoIterator for StatNs {
    type Item = NsCtx;
    type IntoIter = ::std::vec::IntoIter<NsCtx>;

    fn into_iter(self) -> Self::IntoIter {
        self.nses.into_iter()
    }
}

pub struct ListNs {
    pub nstype: Namespace,
    pub nproc: u32,
    pub pid: i32,
    pub ppid: i32,
    pub cmdline: String,
}

pub fn statns_to_nslist(svec: Vec<StatNs>) -> HashMap<u64, ListNs> {
    let mut result_nslist: HashMap<u64, ListNs> = HashMap::new();

    for statns in svec {
        let s_nses = statns.nses;
        for nsctx in s_nses {
            let nsid = nsctx.nsid;

            if result_nslist.contains_key(&nsid) {
                let mut listns = result_nslist.get_mut(&nsid).unwrap();
                *listns = ListNs {
                    nstype: nsctx.nstype,
                    nproc: listns.nproc + 1,
                    pid: statns.stat.pid,
                    ppid: statns.stat.ppid,
                    cmdline: statns.cmdline.clone(),
                };
            } else {
                result_nslist.insert(nsid,
                                     ListNs {
                                         nstype: nsctx.nstype,
                                         nproc: 1,
                                         pid: statns.stat.pid,
                                         ppid: statns.stat.ppid,
                                         cmdline: statns.cmdline.clone(),
                                     });
            }
        }
    }

    result_nslist
}

pub fn ns_str_to_const(nsname: &str) -> Option<Namespace> {
    match nsname.as_ref() {
        "ipc" => Some(Namespace::Ipc),
        "mnt" => Some(Namespace::Mount),
        "net" => Some(Namespace::Net),
        "pid" => Some(Namespace::Pid),
        "user" => Some(Namespace::User),
        "uts" => Some(Namespace::Uts),
        _ => None,
    }
}

pub fn ns_const_to_str<'a>(ns: &Namespace) -> &'a str {
    match ns {
        &Namespace::Ipc => "ipc",
        &Namespace::Mount => "mnt",
        &Namespace::Net => "net",
        &Namespace::Pid => "pid",
        &Namespace::User => "user",
        &Namespace::Uts => "uts",
    }
}

fn ns_symlink_to_ino(symlink_ns: &str) -> u64 {
    let nstype: String;
    let nsino: u64;
    scan!(symlink_ns.bytes() => "{}:[{}]", nstype, nsino);

    nsino
}

fn get_next_pid(entry: &DirEntry) -> Option<i32> {
    let path = entry.path();
    if !path.is_dir() {
        return None;
    }

    let filename = entry.file_name().into_string().unwrap();
    if !filename.chars().nth(0).unwrap().is_digit(10) {
        return None;
    }
    match filename.parse::<i32>() {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

fn get_ns_stat(entry: &DirEntry) -> Option<Vec<NsCtx>> {
    let pathbuf = PathBuf::from(entry.path());
    let mut result_ns: Vec<NsCtx> = Vec::<NsCtx>::new();

    let pathbuf = pathbuf.join("ns");
    if !pathbuf.is_dir() {
        return None;
    }

    match fs::read_dir(pathbuf) {
        Ok(paths) => {
            for entry in paths {
                let entry = entry.unwrap();
                let path = &entry.path();
                if fs::symlink_metadata(path)
                       .unwrap()
                       .file_type()
                       .is_symlink() {
                    let fname_os = entry.file_name();
                    let filename = fname_os.to_str().unwrap();
                    let nsconst = ns_str_to_const(filename);

                    let path_link = path.read_link().unwrap();
                    let fname_path_link = path_link.to_str().unwrap();
                    let ino = ns_symlink_to_ino(fname_path_link);

                    match nsconst {
                        Some(_) => {
                            result_ns.push(NsCtx {
                                               nsid: ino,
                                               nstype: nsconst.unwrap(),
                                           })
                        },
                        None => continue,
                    }
                }
            }
        },
        Err(reason) => {
            println!("Cannot read the directory: {:?}", reason.kind());
        },
    }

    Some(result_ns)
}

pub fn read_proc_to_statns(dir: &Path) -> Result<Vec<StatNs>> {
    let mut result_svec: Vec<StatNs> = Vec::<StatNs>::new();

    if !dir.is_dir() {
        return Ok(result_svec);
    }

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        if let Some(pid) = get_next_pid(&entry) {
            match pid::stat(pid) {
                Ok(ps) => {
                    let mut data = String::new();
                    let mut pathbuf = entry.path();

                    pathbuf = pathbuf.join("cmdline");
                    if !pathbuf.is_file() {
                        continue;
                    }

                    let path = pathbuf.as_path();

                    let mut fh = File::open(path).unwrap();
                    fh.read_to_string(&mut data).unwrap();
                    let statns = StatNs {
                        cmdline: data,
                        stat: ps,
                        nses: get_ns_stat(&entry).unwrap(),
                    };
                    if !statns.nses.is_empty() {
                        result_svec.push(statns);
                    }
                },
                Err(_) => continue,
            }
        }
    }

    result_svec.sort_by_key(|k| k.stat.pid);
    Ok(result_svec)
}

#[cfg(test)]
mod tests {
    use super::{ns_const_to_str, statns_to_nslist, NsCtx, StatNs};
    use procinfo::pid::Stat;
    use unshare::Namespace;

    /// Test the statns
    #[test]
    fn test_statns_to_nslist() {
        let mut listns: Vec<NsCtx> = Vec::<NsCtx>::new();
        let test_nsid = 4000000000u64;
        let test_pid = 12345;
        let test_ppid = 1234;

        listns.push(NsCtx {
                        nsid: test_nsid,
                        nstype: Namespace::Mount,
                    });
        let statns = StatNs {
            cmdline: "/bin/bash".to_string(),
            stat: Stat {
                pid: test_pid,
                ppid: test_ppid,
                ..Default::default()
            },
            nses: listns,
        };

        let mut vec_statns: Vec<StatNs> = Vec::<StatNs>::new();
        vec_statns.push(statns);

        let map_listns = statns_to_nslist(vec_statns);

        assert_eq!(ns_const_to_str(&map_listns[&test_nsid].nstype), "mnt");
        assert_eq!(map_listns[&test_nsid].nproc, 1u32);
        assert_eq!(map_listns[&test_nsid].pid, test_pid);
        assert_eq!(map_listns[&test_nsid].ppid, test_ppid);
        assert_eq!(map_listns[&test_nsid].cmdline, "/bin/bash".to_string());
    }
}
