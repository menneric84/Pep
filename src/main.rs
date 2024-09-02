use std::fs;

use config::Config;
use editor::Editor;

mod config;
mod editor;

fn main() -> anyhow::Result<()> {
    let config_file = Config::path("config.toml");
    eprint!("Config file: {}", config_file.display());
    if !config_file.exists() {
        eprintln!("Config file {} not found", config_file.display());
        std::process::exit(1);
    }

    let toml = fs::read_to_string(config_file)?;
    eprintln!("Read config file: {}", toml);
    let config: Config = toml::from_str(&toml)?;

    let mut editor = Editor::new()?;
    editor.run()?;

    return Ok(());
}
