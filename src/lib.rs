// SPDX-FileCopyrightText: Copyright (c) 2026 Juhan280
// SPDX-License-Identifier: MIT

use bpaf::{Bpaf, OptionParser, Parser};

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

// Copied and modified from bpaf::batteries::verbose_and_quiet_by_number
fn verbosity() -> impl Parser<usize> {
    let offset = 1; // default is 1 for warnings
    let (min, max) = (0, 4);

    let verbose = bpaf::short('v')
        .long("verbose")
        .help("Increase output verbosity, can be used several times")
        .req_flag(())
        .many()
        .map(|v| v.len());

    let quiet = bpaf::short('q')
        .long("quiet")
        .help("Decrease output verbosity, can be used several times")
        .req_flag(())
        .many()
        .map(|v| v.len());

    bpaf::construct!(verbose, quiet).map(move |(v, q)| (offset + v - q).clamp(min, max))
}

// Hide the _parse_cli auto generated function
mod inner {
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
        let help = bpaf::long("help").short('h').help("Print help information");
        let version = bpaf::long("version")
            .short('V')
            .help("Print version information");

        _parse_cli().help_parser(help).version_parser(version)
    }
}

pub use inner::{CliOptions, parse_cli};

#[test]
fn check_bpaf_invariants() {
    parse_cli().check_invariants(false)
}
