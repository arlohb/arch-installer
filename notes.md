# Setting up network

```bash

# Network might just work already if it's wired, check first
ping archlinux.org

# Unblocks all devices
rfkill unblock all

# Initiate scan for networks
# $device will be something like wlan0
iwctl station $device scan

# Connect to wifi
iwctl --passphrase $password station $device connect $ssid

# Setup DHCP
dhcpcd

# Check if that's worked
ping archlinux.org

```

# Install it

```bash

# Set keyboard layout
loadkeys uk
# Set timezone
timedatectl set-timezone Europe/London

# Partitions are already there and right size, just need to format them
mkfs.ext4 /dev/nvme0n1p2
mkfs.fat -F 32 /dev/nvme0n1p4

# Mount new filesystems
mount /dev/nvme0n1p2 /mnt
mount --mkdir /dev/nvme0n1p4 /mnt/boot

# Install base system
pacstrap /mnt base linux-zen linux-firmware
# Install needed packages
pacstrap /mnt vim networkmanager

# Generate the fstab file
genfstab -U /mnt >> /mnt/etc/fstab

```

# Inside the new system with `arch-chroot /mnt`

```bash

# Set the timezone
ln -sf /usr/share/zoneinfo/Europe/London /etc/localetime
hwclock --systohc

# Edit /etc/locale.gen and comment out:
# "en_GB.UTF-8 UTF-8"

# Generate the locales
locale-gen

# Create /etc/locale.conf and type in:
# "LANG=en_GB.UTF-8"

# Create /etc/vconsole.conf and type in:
# "KEYMAP=uk"

# Create /etc/hostname and type in:
# "arlo-arch"

# Set the root password
passwd

# Trying EFISTUB, unsure if it'll work

pacman -Syu efibootmgr amd-ucode

# --disk is just the disk with the efi partition
# --part is the partition number of the efi partition
# The long PARTUUID was found with 'blkid' and is the root partition, not efi
efibootmgr --disk /dev/nvme0n1 --part 4 --create --label "Arch Linux" --loader /vmlinuz-linux-zen --unicode 'root=PARTUUID=5cc58d0e-864c-d249-a497-2b6eec7fdc5d rw quiet splash vt.global_cursor_default=0 initrd=\amd-ucode.img initrd=\initramfs-linux-zen.img' --verbose

# Exit the chroot
exit

# Unmount the new system
umount -R /mnt

# Boot into new system
reboot

```

# General system setup

```bash

# Setup network manager
systemctl enable NetworkManager
systemctl start NetworkManager
nmtui

# Get and run neofetch
pacman -Syu neofetch
neofetch

# Add the arlo user
useradd -m -G wheel arlo
# Set their password
passwd arlo

# Set sudo
pacman -Syu sudo
EDITOR=vim visudo
# uncomment "%wheel ALL=(ALL:ALL) ALL"

# Reboot and login as arlo
sudo reboot

```

# Install yup

```bash

sudo pacman -Syu git base-devel

cd
git clone https://aur.archlinux.org/yup-bin.git
cd yup-bin
makepkg -si
cd ..
rm -rf yup-bin

```

# Get nushell

```bash

# Get nushell
yup nushell

# Check /bin/nu is in /etc/shells, it should be

# Change login shell to nu
chsh -s /bin/nu

# Reboot
sudo reboot

```

# Get dotfiles

```bash

alias etc = sudo git --git-dir=/etc/.cfg --work-tree=/etc
sudo bash -c 'echo ".cfg" >> .gitignore'
sudo git clone --bare https://github.com/arlohb/etc /etc/.cfg

# When doing this, you may need to delete some files it would override
etc checkout

etc config --local status.showUntrackedFiles no

# Reboot so new environment variables are used
sudo reboot

# My dotfiles need these in nushell
yup -Sy shell-color-scripts starship

alias dots = git --git-dir=/home/arlo/.cfg --work-tree=/home/arlo
echo ".cfg" | save .gitignore
git clone --bare https://github.com/arlohb/dotfiles ~/.cfg

# When doing this, you may need to delete some files it would override
dots checkout

dots config --local status.showUntrackedFiles no

# Relogin for new nushell settings
# Errors are expected as we haven't installed dotacat
exit

# Now we have the env vars set by nushell config, we can install rust
yup -Sy rustup
rustup default stable

# And now install dotacat
yup -Sy dotacat

# Relogin
exit

```

# Setup graphics

```bash

# Only on PC
yup -Sy xf86-video-amdgpu vulkan-radeon
yup -Sy xorg

yup -Sy ly
sudo systemctl enable ly

yup -Sy bspwm sxhkd kitty arandr nerd-fonts-fira-code ttf-liberation nitrogen picom rofi

sudo reboot

```
