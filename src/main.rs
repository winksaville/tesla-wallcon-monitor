use base64::{engine::general_purpose::STANDARD, Engine};
use clap::{CommandFactory, FromArgMatches, Parser};
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType},
};
use log::info;
use serde::{Deserialize, Serialize};
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use std::fs::OpenOptions;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::time::Duration;

const COMMANDS: &[&str] = &["lifetime", "version", "vitals", "wifi_status"];

fn min_abbreviation(cmd: &str, all_commands: &[&str]) -> usize {
    for len in 1..=cmd.len() {
        let prefix = &cmd[..len];
        let matches: Vec<_> = all_commands.iter().filter(|c| c.starts_with(prefix)).collect();
        if matches.len() == 1 {
            return len;
        }
    }
    cmd.len()
}

fn format_commands_help() -> String {
    let cmds: Vec<String> = COMMANDS
        .iter()
        .map(|cmd| {
            let min_len = min_abbreviation(cmd, COMMANDS);
            format!("({}){}", &cmd[..min_len], &cmd[min_len..])
        })
        .collect();
    format!("Command: {}", cmds.join(", "))
}

fn init_logging(log_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    let config = ConfigBuilder::new()
        .set_time_format_rfc3339()
        .build();

    WriteLogger::init(LevelFilter::Info, config, file)?;
    Ok(())
}

fn log_json(endpoint: &str, json: &str) {
    info!("{}: {}", endpoint, json);
}

#[derive(Parser)]
#[command(name = "tesla-wallcon-monitor")]
#[command(about = "Monitor a Tesla Wall Connector")]
struct Args {
    /// Name or IP address of the wall connector
    addr: String,

    /// Command to execute
    command: String,

    /// Loop mode: continuously update display (vitals only)
    #[arg(short, long)]
    loop_mode: bool,

    /// Delay in seconds between updates in loop mode
    #[arg(short, long, default_value = "5")]
    delay: u64,

    /// Log file for debug output (JSON data with timestamps)
    #[arg(long)]
    log: Option<PathBuf>,
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

#[derive(Debug, Deserialize, Serialize)]
struct Version {
    firmware_version: String,
    git_branch: String,
    part_number: String,
    serial_number: String,
    web_service: Option<String>,
}

fn get_version(addr: &str) -> Result<Version, Box<dyn std::error::Error>> {
    let url = format!("http://{}/api/1/version", addr);
    let response = reqwest::blocking::get(&url)?;
    let text = response.text()?;
    log_json("version", &text);
    let version: Version = serde_json::from_str(&text)?;
    Ok(version)
}

#[derive(Debug, Deserialize, Serialize)]
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

fn get_wifi_status(addr: &str) -> Result<WifiStatus, Box<dyn std::error::Error>> {
    let url = format!("http://{}/api/1/wifi_status", addr);
    let response = reqwest::blocking::get(&url)?;
    let text = response.text()?;
    log_json("wifi_status", &text);
    let status: WifiStatus = serde_json::from_str(&text)?;
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

#[derive(Debug, Deserialize, Serialize)]
struct Lifetime {
    contactor_cycles: u32,
    contactor_cycles_loaded: u32,
    alert_count: u32,
    thermal_foldbacks: u32,
    avg_startup_temp: f64,
    charge_starts: u32,
    energy_wh: u64,
    connector_cycles: u32,
    uptime_s: u64,
    charging_time_s: u64,
}

fn get_lifetime(addr: &str) -> Result<Lifetime, Box<dyn std::error::Error>> {
    let url = format!("http://{}/api/1/lifetime", addr);
    let response = reqwest::blocking::get(&url)?;
    let text = response.text()?;
    log_json("lifetime", &text);
    let lifetime: Lifetime = serde_json::from_str(&text)?;
    Ok(lifetime)
}

fn format_duration(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let mins = (seconds % 3600) / 60;
    if days > 0 {
        format!("{}d {}h {}m", days, hours, mins)
    } else if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else {
        format!("{}m", mins)
    }
}

fn run_lifetime(addr: &str) {
    match get_lifetime(addr) {
        Ok(lifetime) => {
            println!("Tesla Wall Connector Lifetime Stats:");
            println!("  Charge Starts:      {}", lifetime.charge_starts);
            println!("  Energy Delivered:   {:.2} kWh", lifetime.energy_wh as f64 / 1000.0);
            println!("  Charging Time:      {}", format_duration(lifetime.charging_time_s));
            println!("  Uptime:             {}", format_duration(lifetime.uptime_s));
            println!("  Contactor Cycles:   {}", lifetime.contactor_cycles);
            println!("  Loaded Cycles:      {}", lifetime.contactor_cycles_loaded);
            println!("  Connector Cycles:   {}", lifetime.connector_cycles);
            println!("  Thermal Foldbacks:  {}", lifetime.thermal_foldbacks);
            println!("  Alert Count:        {}", lifetime.alert_count);
            println!("  Avg Startup Temp:   {:.1}°C", lifetime.avg_startup_temp);
        }
        Err(e) => {
            eprintln!("Error fetching lifetime stats: {}", e);
            std::process::exit(1);
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Vitals {
    contactor_closed: bool,
    vehicle_connected: bool,
    session_s: u64,
    grid_v: f64,
    grid_hz: f64,
    vehicle_current_a: f64,
    #[serde(rename = "currentA_a")]
    current_a_a: f64,
    #[serde(rename = "currentB_a")]
    current_b_a: f64,
    #[serde(rename = "currentC_a")]
    current_c_a: f64,
    #[serde(rename = "currentN_a")]
    current_n_a: f64,
    #[serde(rename = "voltageA_v")]
    voltage_a_v: f64,
    #[serde(rename = "voltageB_v")]
    voltage_b_v: f64,
    #[serde(rename = "voltageC_v")]
    voltage_c_v: f64,
    relay_coil_v: f64,
    pcba_temp_c: f64,
    handle_temp_c: f64,
    mcu_temp_c: f64,
    uptime_s: u64,
    input_thermopile_uv: i32,
    prox_v: f64,
    pilot_high_v: f64,
    pilot_low_v: f64,
    session_energy_wh: f64,
    config_status: u32,
    evse_state: u32,
    current_alerts: Vec<serde_json::Value>,
    evse_not_ready_reasons: Vec<u32>,
}

fn get_vitals(addr: &str) -> Result<Vitals, Box<dyn std::error::Error>> {
    let url = format!("http://{}/api/1/vitals", addr);
    let response = reqwest::blocking::get(&url)?;
    let text = response.text()?;
    log_json("vitals", &text);
    let vitals: Vitals = serde_json::from_str(&text)?;
    Ok(vitals)
}

fn format_vitals(vitals: &Vitals) -> String {
    let mut lines = vec![
        "Tesla Wall Connector Vitals:".to_string(),
        format!("  Vehicle Connected:  {}", vitals.vehicle_connected),
        format!("  Contactor Closed:   {}", vitals.contactor_closed),
        format!("  Session Duration:   {}", format_duration(vitals.session_s)),
        format!("  Session Energy:     {:.3} kWh", vitals.session_energy_wh / 1000.0),
        format!("  Vehicle Current:    {:.1} A", vitals.vehicle_current_a),
        String::new(),
        format!("  Grid Voltage:       {:.1} V", vitals.grid_v),
        format!("  Grid Frequency:     {:.3} Hz", vitals.grid_hz),
        format!("  Phase Currents:     A={:.1} B={:.1} C={:.1} N={:.1} A",
            vitals.current_a_a, vitals.current_b_a, vitals.current_c_a, vitals.current_n_a),
        format!("  Phase Voltages:     A={:.1} B={:.1} C={:.1} V",
            vitals.voltage_a_v, vitals.voltage_b_v, vitals.voltage_c_v),
        String::new(),
        format!("  PCBA Temp:          {:.1}°C", vitals.pcba_temp_c),
        format!("  Handle Temp:        {:.1}°C", vitals.handle_temp_c),
        format!("  MCU Temp:           {:.1}°C", vitals.mcu_temp_c),
        String::new(),
        format!("  Pilot High/Low:     {:.1} / {:.1} V", vitals.pilot_high_v, vitals.pilot_low_v),
        format!("  Proximity:          {:.1} V", vitals.prox_v),
        format!("  Relay Coil:         {:.1} V", vitals.relay_coil_v),
        format!("  Thermopile:         {} uV", vitals.input_thermopile_uv),
        String::new(),
        format!("  Uptime:             {}", format_duration(vitals.uptime_s)),
        format!("  EVSE State:         {}", vitals.evse_state),
        format!("  Config Status:      {}", vitals.config_status),
    ];
    if !vitals.current_alerts.is_empty() {
        lines.push(format!("  Current Alerts:     {:?}", vitals.current_alerts));
    }
    if !vitals.evse_not_ready_reasons.is_empty() {
        lines.push(format!("  Not Ready Reasons:  {:?}", vitals.evse_not_ready_reasons));
    }
    lines.join("\n")
}

fn print_vitals(vitals: &Vitals) {
    println!("{}", format_vitals(vitals));
}

fn print_vitals_raw(vitals: &Vitals) {
    print!("{}\r\n", format_vitals(vitals).replace('\n', "\r\n"));
}

fn run_vitals(addr: &str, loop_mode: bool, delay: u64) {
    if loop_mode {
        run_vitals_loop(addr, delay);
    } else {
        match get_vitals(addr) {
            Ok(vitals) => print_vitals(&vitals),
            Err(e) => {
                eprintln!("Error fetching vitals: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn run_vitals_loop(addr: &str, delay: u64) {
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    let mut stdout = stdout();

    loop {
        // Clear screen and move cursor to top
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        match get_vitals(addr) {
            Ok(vitals) => {
                print_vitals_raw(&vitals);
                print!("\r\n  Press ESC or Ctrl+C to exit (updates every {}s)\r\n", delay);
            }
            Err(e) => {
                print!("Error fetching vitals: {}\r\n", e);
                print!("\r\n  Press ESC or Ctrl+C to exit\r\n");
            }
        }
        stdout.flush().unwrap();

        // Check for key press with configured delay timeout
        if event::poll(Duration::from_secs(delay)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Esc => break,
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        break
                    }
                    _ => {}
                }
            }
        }
    }

    terminal::disable_raw_mode().expect("Failed to disable raw mode");
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();
}

fn run_version(addr: &str) {
    match get_version(addr) {
        Ok(version) => {
            println!("Tesla Wall Connector Version Info:");
            println!("  Firmware Version: {}", version.firmware_version);
            println!("  Git Branch:       {}", version.git_branch);
            println!("  Part Number:      {}", version.part_number);
            println!("  Serial Number:    {}", version.serial_number);
            println!("  Web Service:      {}", version.web_service.as_deref().unwrap_or("none"));
        }
        Err(e) => {
            eprintln!("Error fetching version: {}", e);
            std::process::exit(1);
        }
    }
}

fn main() {
    let cmd = Args::command().mut_arg("command", |a| a.help(format_commands_help()));
    let args = Args::from_arg_matches(&cmd.get_matches())
        .expect("Failed to parse arguments");

    // Initialize logging if log file specified
    if let Some(ref log_path) = args.log {
        if let Err(e) = init_logging(log_path) {
            eprintln!("Failed to initialize logging: {}", e);
            std::process::exit(1);
        }
    }

    let command = match match_command(&args.command) {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    match command {
        "lifetime" => run_lifetime(&args.addr),
        "version" => run_version(&args.addr),
        "vitals" => run_vitals(&args.addr, args.loop_mode, args.delay),
        "wifi_status" => run_wifi_status(&args.addr),
        _ => unreachable!(),
    }
}
