// Copyright 2025 Adam McKellar
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::io;

use clap::{CommandFactory, Parser, ValueEnum};
use clap_complete::{Shell, generate};

#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum List {
    /// Long word list
    ///
    /// Every word has ~ 13 bits of entropy.
    Long,
    /// Short word list
    ///
    /// Every word has ~ 10 bits of entropy.
    Short,
    /// Short memorable word list
    ///
    /// Every word has ~ 10 bits of entropy.
    ///
    /// Used words are more common than either the words used in the long
    /// or the short password lists.
    Memorable,
}

/// Diceware password generator cli based on eff password lists
///
/// The security of a password is measured in bits of entropy.
/// If you do not have control over the key derivation function (KDF)
/// used, I recommend entropy higher than 128 bits for encryption purposes and
/// an entropy higher than 80 bits for authentication purposes.
/// If you do have control over the KDF used, you may want to use more secure KDF parameters
/// to stretch the password used. This allows for less secure passwords, guaranteeing
/// a secure encryption.
#[derive(Debug, Clone, Parser)]
#[command(version, about, author)]
pub struct Arguments {
    /// Length of generated password in words
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..65535), group = "strength")]
    pub count: Option<u32>,
    /// Target entropy of password in bits
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..), group = "strength")]
    pub entropy: Option<u32>,
    /// Wordlist to use
    #[arg(short, long, value_enum, default_value_t = List::Short)]
    pub list: List,
    /// Copy password to clipboard instead of printing it
    #[arg(long = "cb")]
    pub copy_clipboard: bool,
    /// Print shell completions for a shell
    #[arg(long, value_enum, exclusive = true, value_name = "SHELL")]
    pub generate_shell_completion: Option<Shell>,
    /// List supported shells for shell completion generation
    #[arg(long, exclusive = true)]
    pub list_supported_shells_for_shell_completion: bool,
    /// Output licensing information to console
    #[arg(long, exclusive = true)]
    pub license: bool,
}

pub fn print_shell_completion_supported_shells() {
    for shell in Shell::value_variants() {
        println!("{shell}");
    }
}

pub fn print_shell_completion(shell: Shell) {
    let mut cmd = Arguments::command();
    let bin_name = cmd.get_name().to_string();
    eprintln!("Generating completion file for {shell:?}...");
    generate(shell, &mut cmd, bin_name, &mut io::stdout());
}
