use std::{
    env,
    fs::{self, File},
    io::BufReader,
    iter,
    path::{Path, PathBuf},
    process,
};

use bpaf::Bpaf;
use ron::ser::PrettyConfig;

mod config;
use config::InputConfig;

#[derive(Bpaf, Clone, Debug, PartialEq, Eq)]
#[bpaf(options, version, fallback_to_usage, generate(parse_command))]
/// Control COSMIC's disable-while-typing touchpad flag.
enum Command {
    #[bpaf(command)]
    /// Check current status of disable-while-typing
    Status,

    #[bpaf(command)]
    /// Toggle disable-while-typing state
    Toggle {
        /// Save the current state before disabling
        save: bool,
    },

    #[bpaf(command)]
    /// Enable disable-while-typing
    Enable {
        /// Save the current state before disabling
        save: bool,
    },

    #[bpaf(command)]
    /// Disable disable-while-typing
    Disable {
        /// Save the current state before disabling
        save: bool,
    },

    #[bpaf(command)]
    /// Restore the previously saved disable-while-typing state
    Restore {
        /// Delete the save state after restoring
        delete: bool,
    },

    #[bpaf(command)]
    /// Print help information
    Help {
        #[bpaf(positional("COMMAND"))]
        command: Option<String>,
    },
}

const DEFAULT: bool = true;

fn main() {
    let command = parse_command().run();

    if let Err(msg) = run(command) {
        eprintln!("Error: {msg}");
        process::exit(1);
    }
}

fn run(command: Command) -> Result<(), String> {
    let home = env::var("HOME")
        .map_err(|_| "Environment variable $HOME is not set. Cannot locate config directory.")?;

    let home = PathBuf::from(home);
    let config_path = home.join(".config/cosmic/com.system76.CosmicComp/v1/input_touchpad");
    let state_dir = home.join(".local/state/cosmic/juhan280.CosmicDwt");
    let state_file = state_dir.join("disable_while_typing");

    let mut config: InputConfig = read_ron_from_file(&config_path)?;

    if let Command::Toggle { save: true }
    | Command::Enable { save: true }
    | Command::Disable { save: true } = command
    {
        fs::create_dir_all(&state_dir).map_err(|e| {
            format!(
                "Failed to create state directory at {}: {}",
                state_dir.display(),
                e
            )
        })?;

        save_ron_to_file(&state_file, &config.disable_while_typing)?;
    }

    match command {
        Command::Status => match config.disable_while_typing {
            Some(true) => println!("Enabled"),
            Some(false) => println!("Disabled"),
            None => println!("Default (Enabled)"),
        },
        Command::Toggle { .. } => {
            let new_val = !config.disable_while_typing.unwrap_or(DEFAULT);
            config.disable_while_typing = Some(new_val);
            save_ron_to_file(&config_path, &config)?;
            println!("Toggled (new value: {new_val})");
        }
        Command::Enable { .. } => {
            config.disable_while_typing = Some(true);
            save_ron_to_file(&config_path, &config)?;
            println!("Enabled");
        }
        Command::Disable { .. } => {
            config.disable_while_typing = Some(false);
            save_ron_to_file(&config_path, &config)?;
            println!("Disabled");
        }
        Command::Restore { delete } => {
            if !state_file.exists() {
                println!("No saved state found. Leaving configuration untouched.");
                return Ok(());
            }

            let saved_state: Option<bool> = read_ron_from_file(&state_file)?;
            config.disable_while_typing = saved_state;
            save_ron_to_file(&config_path, &config)?;
            let new_val = match saved_state {
                Some(true) => "Enabled",
                Some(false) => "Disabled",
                None => "Default",
            };
            println!("Restored (new value: {new_val})");

            if delete {
                fs::remove_file(&state_file).map_err(|e| {
                    format!(
                        "Failed to delete save state file at {}:\n  {e}",
                        state_file.display()
                    )
                })?;
            }
        }
        Command::Help { command } => {
            let args: Vec<_> = command
                .as_deref()
                .into_iter()
                .chain(iter::once("--help"))
                .collect();
            parse_command()
                .run_inner(&*args)
                .unwrap_err()
                .print_message(80);
        }
    }

    Ok(())
}

fn read_ron_from_file<T: serde::de::DeserializeOwned>(config_path: &Path) -> Result<T, String> {
    let file = File::open(config_path).map_err(|e| {
        format!(
            "Failed to read RON file at {}:\n  {e}",
            config_path.display(),
        )
    })?;

    ron::de::from_reader(BufReader::new(file))
        .map_err(|e| format!("Failed to parse RON file format (corrupt RON layout):\n  {e}"))
}

fn save_ron_to_file<T: ?Sized + serde::Serialize>(path: &Path, data: &T) -> Result<(), String> {
    let str = ron::ser::to_string_pretty(&data, PrettyConfig::new())
        .map_err(|e| format!("Failed to format configuration back to RON: {}", e))?;

    fs::write(path, str).map_err(|e| {
        format!(
            "Failed to save modifications to file disk at {}: {e}",
            path.display(),
        )
    })?;

    Ok(())
}
