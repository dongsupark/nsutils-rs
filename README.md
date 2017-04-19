# lsns

_Command-line utilities written in Rust for making use of Linux namespaces._

*highly experimental*

Currently supported binaries:

* [x] lsns
* [ ] nsenter

## lsns

``lsns`` is a simple command-line tool for listing all Linux namespaces, just like the lsns included in util-linux package which is available under most Linux distros.
    
``lsns`` depends on existing rust packages such as unshare, procinfo, etc.

### Usage

Run cargo to install lsns.

```rust
$ cargo install lsns
```

Run lsns.

```
$ ./target/debug/lsns
NSID NSTYPE NPROC PID PPID COMMAND
4026532782 pid 1 2057 2033 /opt/google/chrome/nacl_helper
4026531839 ipc 100 2991 2988 /bin/bash
4026531840 mount 100 2991 2988 /bin/bash
4026532784 pid 2 2033 1817 /opt/google/chrome/chrome --type=zygote
4026532975 user 1 2057 2033 /opt/google/chrome/nacl_helper
4026531837 user 90 2991 2988 /bin/bash
4026532881 net 1 2057 2033 /opt/google/chrome/nacl_helper
4026531838 uts 100 2991 2988 /bin/bash
```

### Dependencies

```toml
[dependencies]
lsns = "0.0.1"
```

# License

Licensed under either of

* Apache License, Version 2.0, (./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (./LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

