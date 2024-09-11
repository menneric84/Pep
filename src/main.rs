use std::fs::{self};

use config::Config;
use editor::Editor;

mod config;
mod editor;
mod window;

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

    match Editor::new(config, "/home/rick/repos/pep/src/test.txt".to_string()) {
        Err(e) => eprintln!("Error creating editor: {}", e),
        Ok(mut e) => e.run()?,
    }

    return Ok(());
}
