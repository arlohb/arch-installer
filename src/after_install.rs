use crate::{turn_off_beeps, Config};

pub fn after_install(_config: Config) -> Result<(), std::io::Error> {
    println!("Turning off beeps...");
    turn_off_beeps()?;

    // systemctl enable NetworkManager
    // systemctl start NetworkManager
    // nmtui

    Ok(())
}
