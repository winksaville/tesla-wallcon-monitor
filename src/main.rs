use base64::{engine::general_purpose::STANDARD, Engine};
use clap::Parser;
use serde::Deserialize;

const COMMANDS: &[&str] = &["version", "wifi_status"];

#[derive(Parser)]
#[command(name = "tesla-wallcon-monitor")]
#[command(about = "Monitor a Tesla Wall Connector")]
struct Args {
    /// Name or IP address of the wall connector
    addr: String,

    /// Command to execute (can be abbreviated)
    command: String,
}

fn match_command(input: &str) -> Result<&'static str, String> {
    let matches: Vec<&str> = COMMANDS
        .iter()
        .filter(|cmd| cmd.starts_with(input))
        .copied()
        .collect();

    match matches.len() {
        0 => Err(format!(
            "Unknown command '{}'. Available commands: {}",
            input,
            COMMANDS.join(", ")
        )),
        1 => Ok(matches[0]),
        _ => Err(format!(
            "Ambiguous command '{}'. Matches: {}",
            input,
            matches.join(", ")
        )),
    }
}

#[derive(Debug, Deserialize)]
struct Version {
    firmware_version: String,
    git_branch: String,
    part_number: String,
    serial_number: String,
    web_service: String,
}

fn get_version(addr: &str) -> Result<Version, reqwest::Error> {
    let url = format!("http://{}/api/1/version", addr);
    let response = reqwest::blocking::get(&url)?;
    let version: Version = response.json()?;
    Ok(version)
}

#[derive(Debug, Deserialize)]
struct WifiStatus {
    wifi_ssid: String,
    wifi_signal_strength: i32,
    wifi_rssi: i32,
    wifi_snr: i32,
    wifi_connected: bool,
    wifi_infra_ip: String,
    internet: bool,
    wifi_mac: String,
}

fn get_wifi_status(addr: &str) -> Result<WifiStatus, reqwest::Error> {
    let url = format!("http://{}/api/1/wifi_status", addr);
    let response = reqwest::blocking::get(&url)?;
    let status: WifiStatus = response.json()?;
    Ok(status)
}

fn decode_ssid(encoded: &str) -> String {
    STANDARD
        .decode(encoded)
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .unwrap_or_else(|| encoded.to_string())
}

fn run_wifi_status(addr: &str) {
    match get_wifi_status(addr) {
        Ok(status) => {
            println!("Tesla Wall Connector WiFi Status:");
            println!("  SSID:            {}", decode_ssid(&status.wifi_ssid));
            println!("  Connected:       {}", status.wifi_connected);
            println!("  Signal Strength: {}%", status.wifi_signal_strength);
            println!("  RSSI:            {} dBm", status.wifi_rssi);
            println!("  SNR:             {} dB", status.wifi_snr);
            println!("  IP Address:      {}", status.wifi_infra_ip);
            println!("  Internet:        {}", status.internet);
            println!("  MAC Address:     {}", status.wifi_mac);
        }
        Err(e) => {
            eprintln!("Error fetching wifi status: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_version(addr: &str) {
    match get_version(addr) {
        Ok(version) => {
            println!("Tesla Wall Connector Version Info:");
            println!("  Firmware Version: {}", version.firmware_version);
            println!("  Git Branch:       {}", version.git_branch);
            println!("  Part Number:      {}", version.part_number);
            println!("  Serial Number:    {}", version.serial_number);
            println!("  Web Service:      {}", version.web_service);
        }
        Err(e) => {
            eprintln!("Error fetching version: {}", e);
            std::process::exit(1);
        }
    }
}

fn main() {
    let args = Args::parse();

    let command = match match_command(&args.command) {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    match command {
        "version" => run_version(&args.addr),
        "wifi_status" => run_wifi_status(&args.addr),
        _ => unreachable!(),
    }
}
