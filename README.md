# tesla-wallcon-monitor

A CLI tool to monitor a Tesla Wall Connector.

## Building

```bash
$ cargo build --release
   Compiling proc-macro2 v1.0.103
   Compiling quote v1.0.42
   Compiling unicode-ident v1.0.22
   ..
   Compiling hyper-tls v0.6.0
   Compiling reqwest v0.12.26
   Compiling tesla-wallcon-monitor v0.1.0 (/home/wink/data/prgs/tesla/tesla-wallcon-monitor)
    Finished `release` profile [optimized] target(s) in 26.63s
```

## Usage

```bash
$ cargo run -- --help
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/tesla-wallcon-monitor --help`
Monitor a Tesla Wall Connector

Usage: tesla-wallcon-monitor <ADDR>

Arguments:
  <ADDR>  Name or IP address of the wall connector

Options:
  -h, --help  Print help
```

### Example

```bash
$ tesla-wallcon-monitor 192.168.1.221
Tesla Wall Connector Version Info:
  Firmware Version: 24.36.3+gf5ba241b03c449
  Git Branch:       heads/24.36.3
  Part Number:      1529455-02-D
  Serial Number:    TWC123456789
  Web Service:      0.1.0
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
