use clap::Parser;
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "tesla-wallcon-monitor")]
#[command(about = "Monitor a Tesla Wall Connector")]
struct Args {
    /// Name or IP address of the wall connector
    addr: String,
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

fn main() {
    let args = Args::parse();

    match get_version(&args.addr) {
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
