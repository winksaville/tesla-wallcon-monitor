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
tesla-wallcon-monitor <ADDR> <COMMAND>
```

Commands can be abbreviated (e.g., `v` or `ver` for `version`).

### Commands

- `version` - Display firmware and device information
- `wifi_status` - Display WiFi connection status

### Examples

```bash
$ tesla-wallcon-monitor 192.168.1.221 version
Tesla Wall Connector Version Info:
  Firmware Version: 25.34.1+ge48cc9be91ebc7
  Git Branch:       HEAD
  Part Number:      1529455-02-D
  Serial Number:    TWC123456789
  Web Service:      0.1.0

$ tesla-wallcon-monitor 192.168.1.221 wifi_status
Tesla Wall Connector WiFi Status:
  SSID:            MyNetwork
  Connected:       true
  Signal Strength: 66%
  RSSI:            -57 dBm
  SNR:             36 dB
  IP Address:      192.168.1.221
  Internet:        true
  MAC Address:     54:F8:F0:0A:30:AA
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
