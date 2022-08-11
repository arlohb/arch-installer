# How to use

## Get an arch linux image

[A list of mirrors](https://archlinux.org/download/)

- Download the ISO
- Somehow flash it to a USB pen
- Etcher is a super easy way to do this

## Download the script

- Put the built executable and the config.toml onto another USB drive

## Run the script

- Boot to the USB
- List drives with:
- $ fdisk -l
- Mount the other USB pen with:
- $ mount --mkdir /dev/{device} /media/script
- Run the script:
- $ cd /media/script
- $ ./arch_installer config.toml
