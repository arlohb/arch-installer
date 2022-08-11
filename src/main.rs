#![warn(clippy::unwrap_used, clippy::pedantic, clippy::nursery)]

mod config;
use std::process::Command;

pub use config::*;

use cmd_lib::run_cmd;

pub fn cd(path: &str) {
    std::env::set_current_dir(path).expect("Failed to change directory");
}

#[must_use]
pub fn has_internet() -> bool {
    Command::new("ping")
        // Only run for 5 seconds
        .arg("-w 5")
        .arg("www.google.com")
        .spawn()
        .expect("Failed to start ping command")
        .wait()
        .expect("Failed to wait for ping command")
        .success()
}

fn main() -> Result<(), std::io::Error> {
    println!("Loading config...");
    let Config {
        keymap,
        timezone,
        wifi,
    } = Config::load();

    println!("Loading keymap...");
    run_cmd!(loadkeys $keymap)?;

    println!("Checking internet...");
    if !has_internet() {
        // Need to connect to the internet

        // Unblock devices
        run_cmd!(rfkill unblock all)?;

        if let Some(Wifi {
            device,
            ssid,
            password,
        }) = wifi
        {
            println!("Connecting to wifi...");

            // Connect to wifi
            run_cmd!(iwctl --passphrase $password station $device connect $ssid)?;

            // Setup DHCP
            run_cmd!(dhcpcd)?;

            // Check if that worked
            assert!(has_internet(), "Failed to connect to wifi");
        } else {
            panic!("Wired network not supported yet");
        }
    }

    println!("Setting timezone...");
    run_cmd!(timedatectl set-timezone $timezone)?;

    Ok(())
}
