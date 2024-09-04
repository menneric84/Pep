use std::{fs::{self, OpenOptions}, io::Read};

use config::Config;
use editor::Editor;

mod config;
mod editor;

fn main() -> anyhow::Result<()> {
    let config_file = Config::path("config.toml");
    eprintln!("Config file: {}", config_file.display());

    if !config_file.exists() {
        eprintln!("Config file {} not found", config_file.display());
        std::process::exit(1);
    }

    let toml = fs::read_to_string(config_file)?;

    let config: Config = toml::from_str(&toml)?;

    let args = std::env::args();

    match args.max() {
        Some(arg) => {
            let mut file = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(arg);

            if file.is_err() {
                eprintln!("Error opening file: {}", file.err().unwrap());
                std::process::exit(1);
            }

            match Editor::new(config, file?) {
                Err(e) => eprintln!("Error creating editor: {}", e),
                Ok(mut e) => e.run()?,
            }
        }
        None => {
            eprintln!("No fileName provided");
        }
    }

    return Ok(());
}
