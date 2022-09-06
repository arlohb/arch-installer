#![warn(clippy::unwrap_used, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::let_underscore_drop,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc
)]

mod after_install;
mod config;
mod install;
mod utils;

pub use after_install::*;
pub use config::*;
pub use install::*;
pub use utils::*;

use clap::Parser;

#[derive(clap::ValueEnum, Clone)]
pub enum Stage {
    Install,
    AfterInstall,
}

#[derive(Parser)]
#[clap(author = "Arlo Blythe", about = "An arch install script")]
pub struct Args {
    /// The path to the config file
    pub config_file: String,
    /// The stage to run
    #[clap(value_enum)]
    pub stage: Stage,
}

#[allow(clippy::too_many_lines)]
fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    println!("Loading config...");
    let config = Config::load(args.config_file);

    match args.stage {
        Stage::Install => install(config)?,
        Stage::AfterInstall => after_install(config)?,
    };

    Ok(())
}
