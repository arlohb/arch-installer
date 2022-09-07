# My install on main pc

```bash

# Network is automatically setup as it's just ethernet

loadkeys uk

# Partitions are already there and right size, just need to format them
mkfs.ext4 /dev/nvme0n1p2
mkfs.fat -F 32 /dev/nvme0n1p4

mount /dev/nvme0n1p2 /mnt
mount --mkdir /dev/nvme0n1p4 /mnt/boot

pacstrap /mnt base linux-zen linux-firmware
pacstrap /mnt vim networkmanager

genfstab -U /mnt >> /mnt/etc/fstab

```

## All this is in `arch-chroot /mnt`

```bash

ln -sf /usr/share/zoneinfo/Europe/London /etc/localetime

hwclock --systohc

# Edit /etc/locale.gen and comment out:
# "en_GB.UTF-8 UTF-8"

locale-gen

# Create /etc/locale.conf and type in:
# "LANG=en_GB.UTF-8"

# Create /etc/vconsole.conf and type in:
# "KEYMAP=uk"

# Create /etc/hostname and type in:
# "arlo-arch"

passwd

# Trying EFISTUB, unsure if it'll work

pacman -Syu efifbootmgr

# --disk is just the disk with the efi partition
# --part is the partition number of the efi partition
# The long PARTUUID was found with 'blkid' and is the root partition, not efi
efibootmgr --disk /dev/nvme0n1 --part 4 --create --label "Arch Linux" --loader /vmlinuz-linux-zen --unicode 'root=PARTUUID=5cc58d0e-864c-d249-a497-2b6eec7fdc5d rw initrd=\amd-ucode.img initrd=\initramfs-linux-zen.img' --verbose

```

## After rebooting into new system

```bash

systemctl enable NetworkManager
systemctl start NetworkManager
nmtui

pacman -Syu neofetch
neofetch

useradd -m -G wheel arlo
passwd arlo

pacman -Syu sudo
EDITOR=vim visudo
# uncomment "%wheel ALL=(ALL:ALL) ALL"

```

## As the arlo user

```bash

sudo pacman -Syu git base-devel

cd
git clone https://aur.archlinux.org/yup-bin.git
cd yup-bin
makepkg -si
cd ..
rm -rf yup-bin

# Change these values:
# "print_pkgbuild": false,
# "ask_pkgbuild": false,
# "ask_redo": false,
# "vim_keybindings": true
vim .config/yup/config.json

yup -Sy mesa xf86-video-amdgpu vulkan-radeon
yup -Sy xorg

yup -Sy ly
sudo systemctl enable ly

yup -Sy bspwm sxhkd kitty arandr
mkdir .config/bspwm
cp /usr/share/doc/bspwm/examples/bspwmrc .config/bspwm
mkdir .config/sxhkd
cp /usr/share/doc/bspwm/examples/sxhkdrc .config/sxhkd

# Change terminal to kitty in sxhkdrc

# Kitty doesn't support bitmaps fonts by default
yup -Sy nerd-fonts-fira-code

sudo reboot

```
