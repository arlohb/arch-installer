use cmd_lib::run_cmd;

pub fn cd(path: &str) {
    std::env::set_current_dir(path).expect("Failed to change directory");
}

#[must_use]
pub fn has_internet() -> bool {
    run_cmd!(ping -w 5 www.google.com).is_ok()
}

pub fn turn_off_beeps() -> Result<(), std::io::Error> {
    if run_cmd!(rmmod pcspkr).is_ok() {
        run_cmd!(echo "blacklist pcspkr" >>/etc/modprobe.d/blacklist.conf)?;
    }

    Ok(())
}
