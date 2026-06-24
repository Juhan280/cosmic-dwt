use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

use bpaf::Bpaf;
use ron::ser::PrettyConfig;

mod config;
use config::InputConfig;

#[derive(Bpaf, Clone, Debug, PartialEq, Eq)]
#[bpaf(options, version)]
/// Control COSMIC's disable-while-typing touchpad flag.
enum Command {
    #[bpaf(command)]
    /// Check current status of disable-while-typing
    Status,
    #[bpaf(command)]
    /// Toggle disable-while-typing state
    Toggle,
    #[bpaf(command)]
    /// Enable disable-while-typing
    Enable,
    #[bpaf(command)]
    /// Disable disable-while-typing
    Disable,
}

const DEFAULT: bool = true;

fn main() {
    let command = command().run();

    if let Err(msg) = run(command) {
        eprintln!("Error: {msg}");
        process::exit(1);
    }
}

fn run(command: Command) -> Result<(), String> {
    let home = env::var("HOME")
        .map_err(|_| "Environment variable $HOME is not set. Cannot locate config directory.")?;

    let path = PathBuf::from(home).join(".config/cosmic/com.system76.CosmicComp/v1/input_touchpad");

    let config_string = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read config file at {}:\n  {}", path.display(), e))?;

    let mut config: InputConfig = ron::from_str(&config_string).map_err(|e| {
        format!(
            "Failed to parse COSMIC config file format (corrupt RON layout):\n  {}",
            e
        )
    })?;

    match command {
        Command::Status => match config.disable_while_typing {
            Some(true) => println!("Enabled"),
            Some(false) => println!("Disabled"),
            None => println!("Default (Enabled)"),
        },
        Command::Toggle => {
            let new_val = match config.disable_while_typing {
                Some(val) => !val,
                None => !DEFAULT,
            };
            config.disable_while_typing = Some(new_val);
            save_config(&path, &config)?;
            println!("Toggled (new value: {new_val})");
        }
        Command::Enable => {
            config.disable_while_typing = Some(true);
            save_config(&path, &config)?;
            println!("Enabled");
        }
        Command::Disable => {
            config.disable_while_typing = Some(false);
            save_config(&path, &config)?;
            println!("Disabled");
        }
    }

    Ok(())
}

fn save_config(path: &Path, config: &InputConfig) -> Result<(), String> {
    let str = ron::ser::to_string_pretty(&config, PrettyConfig::new())
        .map_err(|e| format!("Failed to format configuration back to RON: {}", e))?;

    fs::write(path, str).map_err(|e| {
        format!(
            "Failed to save modifications to file disk at {}: {}",
            path.display(),
            e
        )
    })?;

    Ok(())
}
