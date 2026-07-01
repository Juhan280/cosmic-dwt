// SPDX-FileCopyrightText: Copyright (c) 2026 Juhan280
// SPDX-License-Identifier: MIT

use bpaf::{Bpaf, OptionParser, Parser, batteries::verbose_and_quiet_by_number};

#[derive(Bpaf, Clone, Debug)]
#[bpaf(generate(parse_command))]
pub enum Command {
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

// Hide the _parse_cli auto generated function
mod cli {
    use super::*;

    #[derive(Bpaf, Debug)]
    #[bpaf(options, version, fallback_to_usage, generate(_parse_cli))]
    /// Control COSMIC's disable-while-typing touchpad flag.
    pub struct CliOptions {
        #[bpaf(external)]
        pub verbosity: usize,

        #[bpaf(external(parse_command))]
        pub command: Command,
    }

    pub fn parse_cli() -> OptionParser<CliOptions> {
        let help_parser = bpaf::long("help").short('h').help("Print help information");
        let version_parser = bpaf::long("version")
            .short('V')
            .help("Print version information");

        _parse_cli()
            .help_parser(help_parser)
            .version_parser(version_parser)
    }
}

pub use cli::{CliOptions, parse_cli};
