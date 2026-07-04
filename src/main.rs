// SPDX-FileCopyrightText: Copyright (c) 2026 Juhan280
// SPDX-License-Identifier: MIT

use std::{
    env, fs, iter,
    path::{Path, PathBuf},
    process,
};

use cosmic_dwt::{Command, parse_cli};

mod config;
mod util;

use config::InputConfig;
use util::{read_ron_from_file, save_ron_to_file};

const DEFAULT: bool = true;

fn main() {
    let cli = parse_cli().run();

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

    let mut config: InputConfig = read_ron_from_file(config_path)?;

    // save original config when --save is specified
    if let Command::Toggle { save: true }
    | Command::Enable { save: true }
    | Command::Disable { save: true } = command
    {
        log::debug!("Creating state directory at {}", state_file.display());
        fs::create_dir_all(state_dir).map_err(|e| {
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
            save_ron_to_file(config_path, &config)?;
            println!("Toggled (new value: {new_val})");
        }
        Command::Enable { .. } => {
            config.disable_while_typing = Some(true);
            save_ron_to_file(config_path, &config)?;
            println!("Enabled");
        }
        Command::Disable { .. } => {
            config.disable_while_typing = Some(false);
            save_ron_to_file(config_path, &config)?;
            println!("Disabled");
        }
        Command::Restore { delete } => {
            if !state_file.exists() {
                println!("No saved state found. Leaving configuration untouched.");
                return Ok(());
            }

            let saved_state: Option<bool> = read_ron_from_file(&state_file)?;
            config.disable_while_typing = saved_state;
            save_ron_to_file(config_path, &config)?;
            let new_val = match saved_state {
                Some(true) => "Enabled",
                Some(false) => "Disabled",
                None => "Default",
            };
            println!("Restored (new value: {new_val})");

            if delete {
                log::info!("Deleting save state file at {}", state_file.display());
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
