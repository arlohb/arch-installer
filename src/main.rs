#![warn(clippy::unwrap_used, clippy::pedantic, clippy::nursery)]

mod config;
use std::io::BufRead;

pub use config::*;

use cmd_lib::run_cmd;

pub fn cd(path: &str) {
    std::env::set_current_dir(path).expect("Failed to change directory");
}

#[must_use]
pub fn has_internet() -> bool {
    run_cmd!(ping -w 5 www.google.com).is_ok()
}

fn main() -> Result<(), std::io::Error> {
    println!("Loading config...");
    let Config {
        keymap,
        timezone,
        locale,
        disk_path,
        hostname,
        wifi,
    } = Config::load();

    println!("Turning off beeps...");
    run_cmd!(rmmod pcspkr)?;
    run_cmd!(echo "blacklist pcspkr" >>/etc/modprobe.d/blacklist.conf)?;

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

    println!("Setting up partitions...");
    let gpt_cfg = gpt::GptConfig::new().writable(true);
    let mut disk = gpt_cfg.open(&disk_path)?;

    for id in disk
        .partitions()
        .iter()
        .map(|(id, _)| *id)
        .collect::<Vec<_>>()
    {
        disk.remove_partition(Some(id), None)?;
    }

    let efi_part_id = disk.add_partition("EFI", 512, gpt::partition_types::EFI, 0, None)?;
    let efi_part_device = format!("{disk_path}{efi_part_id}");

    let free_space = disk
        .find_free_sectors()
        .into_iter()
        .map(|(_start, size)| size)
        .max()
        .expect("No free sectors")
        * u64::from(*disk.logical_block_size());

    let fs_part_id =
        disk.add_partition("Linux", free_space, gpt::partition_types::LINUX_FS, 0, None)?;
    let fs_part_device = format!("{disk_path}{fs_part_id}");

    disk.write()?;

    println!("Creating filesystem...");
    run_cmd!(mkfs.ext4 $fs_part_device)?;
    run_cmd!(mkfs.fat -F 32 $efi_part_device)?;

    println!("Mounting filesystem...");
    run_cmd!(mount $fs_part_device /mnt)?;
    run_cmd!(mount --mkdir $efi_part_device /mnt/boot)?;

    println!("Installing base system...");
    run_cmd!(pacstrap /mnt base linux-zen linux-firmware)?;

    println!("Installing basic programs...");
    run_cmd!(pacstrap /mnt vim networkmanager)?;

    println!("Generating fstab...");
    run_cmd!(genfstab -U /mnt >> /mnt/etc/fstab)?;

    println!("Setting time zone...");
    run_cmd!(arch-chroot /mnt ln -sf /usr/share/zoneinfo/$timezone /etc/localtime)?;
    run_cmd!(arch-chroot /mnt hwclock --systohc)?;

    println!("Setting locale...");

    let locale_gen = std::fs::read_to_string("/mnt/etc/locale.gen")?;

    let locale_gen = locale_gen
        .lines()
        .map(|line| {
            if line.contains(&locale) {
                line.trim_start_matches('#')
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    std::fs::write("/mnt/etc/locale.gen", locale_gen)?;

    run_cmd!(arch-chroot /mnt locale-gen)?;

    std::fs::write("/mnt/etc/locale.conf", format!("LANG={locale}"))?;
    std::fs::write("/mnt/etc/vconsole.conf", format!("KEYMAP={keymap}"))?;

    println!("Setting hostname...");
    std::fs::write("/mnt/etc/hostname", hostname)?;

    Ok(())
}
