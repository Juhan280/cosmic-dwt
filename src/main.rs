// SPDX-FileCopyrightText: Copyright (c) 2026 Juhan280
// SPDX-License-Identifier: MIT

use std::{
    env,
    fs::{self, File},
    io::BufReader,
    iter,
    path::{Path, PathBuf},
    process,
};

use bpaf::{Bpaf, OptionParser, Parser, batteries::verbose_and_quiet_by_number};
use ron::ser::PrettyConfig;

mod config;
use config::InputConfig;

const DEFAULT: bool = true;

#[derive(Bpaf, Clone, Debug)]
#[bpaf(generate(parse_command))]
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

fn verbosity() -> impl Parser<usize> {
    verbose_and_quiet_by_number(1, 0, 4).map(|v| v as usize)
}

#[derive(Bpaf, Debug)]
#[bpaf(options, version, fallback_to_usage, generate(_parse_cli))]
/// Control COSMIC's disable-while-typing touchpad flag.
struct CliOptions {
    #[bpaf(external)]
    verbosity: usize,

    #[bpaf(external(parse_command))]
    command: Command,
}

fn parse_cli() -> OptionParser<CliOptions> {
    let help_parser = bpaf::long("help").short('h').help("Print help information");
    let version_parser = bpaf::long("version")
        .short('V')
        .help("Print version information");

    _parse_cli()
        .help_parser(help_parser)
        .version_parser(version_parser)
}

fn main() {
    let cli = parse_cli().run();
    dbg!(cli.verbosity);

    stderrlog::new()
        .module(module_path!())
        .verbosity(cli.verbosity)
        .init()
        .expect("Failed to setup logger");

    let config_base = env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
        .unwrap_or_else(|| {
            log::error!("Error: Neither $XDG_CONFIG_HOME nor $HOME is set.");
            process::exit(1);
        });

    let state_base = env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/state")))
        .unwrap_or_else(|| {
            log::error!("Error: Neither $XDG_STATE_HOME nor $HOME is set.");
            process::exit(1);
        });

    let config_path = config_base.join("cosmic/com.system76.CosmicComp/v1/input_touchpad");
    let state_dir = state_base.join("cosmic/juhan280.CosmicDwt");
    let state_file = "disable_while_typing";

    if run(cli.command, &config_path, &state_dir, state_file).is_err() {
        process::exit(1);
    }
}

fn run(command: Command, config_path: &Path, state_dir: &Path, state_file: &str) -> Result<(), ()> {
    let state_file = state_dir.join(state_file);

    let mut config: InputConfig = read_ron_from_file(&config_path)?;

    // save original config when --save is specified
    if let Command::Toggle { save: true }
    | Command::Enable { save: true }
    | Command::Disable { save: true } = command
    {
        fs::create_dir_all(&state_dir).map_err(|e| {
            log::error!(
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
            None => println!("Enabled (Default)"),
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
                    log::error!(
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
            parse_cli().run_inner(&*args).unwrap_err().print_message(80);
        }
    }

    Ok(())
}

fn read_ron_from_file<T: serde::de::DeserializeOwned>(config_path: &Path) -> Result<T, ()> {
    log::info!("Reading RON file from: {}", config_path.display());
    let file = File::open(config_path).map_err(|e| {
        log::error!(
            "Failed to read RON file from {}:\n  {e}",
            config_path.display(),
        )
    })?;
    ron::de::from_reader(BufReader::new(file))
        .map_err(|e| log::error!("Failed to parse RON file format (corrupt RON layout):\n  {e}"))
}

fn save_ron_to_file<T: ?Sized + serde::Serialize>(path: &Path, data: &T) -> Result<(), ()> {
    let str = ron::ser::to_string_pretty(&data, PrettyConfig::new())
        .map_err(|e| log::error!("Failed to format configuration back to RON: {}", e))?;

    let temp_path = path.with_extension("tmp");

    log::trace!("Writing data to temp_file at: {}", temp_path.display());
    fs::write(&temp_path, str).map_err(|e| {
        log::error!(
            "Failed to write temporary configuration file at {}: {e}",
            temp_path.display(),
        )
    })?;

    log::trace!(
        "Attempting to atomically save modifications to {}",
        path.display()
    );
    fs::rename(&temp_path, path).map_err(|e| {
        log::error!(
            "Failed to atomically save modifications to disk at {}: {e}",
            path.display(),
        )
    })?;

    Ok(())
}
