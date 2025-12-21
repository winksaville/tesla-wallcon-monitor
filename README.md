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
tesla-wallcon-monitor [OPTIONS] <ADDR> <COMMAND>
```

### Options

- `-l, --loop-mode` - Continuously update display (vitals only). Press ESC or Ctrl+C to exit.
- `-d, --delay <SECONDS>` - Delay between updates in loop mode (default: 5).
- `--log <FILE>` - Log raw JSON responses with timestamps to a file for later processing.

### Commands

Commands can be abbreviated to their minimum unique prefix:

| Abbrev | Command      | Description                              |
|--------|--------------|------------------------------------------|
| l      | lifetime     | Display lifetime statistics              |
| ve     | version      | Display firmware and device information  |
| vi     | vitals       | Display real-time charging status        |
| w      | wifi_status  | Display WiFi connection status           |

### Examples

```bash
$ tesla-wallcon-monitor --help
Monitor a Tesla Wall Connector

Usage: tesla-wallcon-monitor [OPTIONS] <ADDR> <COMMAND>

Arguments:
  <ADDR>     Name or IP address of the wall connector
  <COMMAND>  Command: (l)ifetime, (ve)rsion, (vi)tals, (w)ifi_status

Options:
  -l, --loop-mode      Loop mode: continuously update display (vitals only)
  -d, --delay <DELAY>  Delay in seconds between updates in loop mode [default: 5]
      --log <LOG>      Log file for debug output (JSON data with timestamps)
  -h, --help           Print help

$ tesla-wallcon-monitor 192.168.1.221 vitals
Tesla Wall Connector Vitals:
  Vehicle Connected:  false
  Contactor Closed:   false
  Session Duration:   0m
  Session Energy:     0.000 kWh
  Vehicle Current:    0.0 A

  Grid Voltage:       245.9 V
  Grid Frequency:     59.867 Hz
  Phase Currents:     A=0.0 B=0.0 C=0.0 N=0.0 A
  Phase Voltages:     A=0.2 B=0.0 C=0.2 V

  PCBA Temp:          21.9°C
  Handle Temp:        19.5°C
  MCU Temp:           29.3°C

  Pilot High/Low:     11.8 / 11.8 V
  Proximity:          0.5 V
  Relay Coil:         0.0 V
  Thermopile:         -84 uV

  Uptime:             1d 4h 15m
  EVSE State:         1
  Config Status:      5
  Not Ready Reasons:  [4, 8]

$ tesla-wallcon-monitor 192.168.1.221 lifetime
Tesla Wall Connector Lifetime Stats:
  Charge Starts:      1054
  Energy Delivered:   5276.62 kWh
  Charging Time:      45d 13h 54m
  Uptime:             588d 0h 18m
  Contactor Cycles:   1054
  Loaded Cycles:      66
  Connector Cycles:   483
  Thermal Foldbacks:  0
  Alert Count:        2243
  Avg Startup Temp:   0.0°C

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
