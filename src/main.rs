// Copyright 2025 Adam McKellar
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use clap::Parser;
use colored::Colorize;
use error_stack::{Report, ResultExt};
use license_fetcher::read_package_list_from_out_dir;
use thiserror::Error;

use crate::{
    cli::{Arguments, List},
    clipboard::copy_to_clipboard_and_delete,
    decode::decode_password_lists,
    entropy::{calculate_entropy, count_from_entropy},
    generation::diceware_password,
};

mod cli;
mod clipboard;
mod decode;
mod entropy;
mod generation;
mod password_lists;

use mimalloc::MiMalloc;
use zeroizing_alloc::ZeroAlloc;

#[global_allocator]
static ALLOC: ZeroAlloc<MiMalloc> = ZeroAlloc(MiMalloc);

fn print_licenses() {
    let package_list = read_package_list_from_out_dir!().unwrap();
    println!("{}", package_list);
}

#[derive(Debug, Clone, Error)]
enum MainError {
    #[error("Failed to decode password lists")]
    DecodeLists,
    #[error("Failed to dice")]
    Dice,
    #[error("Failed to calc entropy")]
    EntropyCalculation,
    #[error("Failed to copy")]
    Clipboard,
    #[error("Failed downcast of number")]
    Downcast,
}

fn main() -> Result<(), Report<MainError>> {
    let cli = Arguments::parse();

    if cli.license {
        print_licenses();
        return Ok(());
    }

    let password_lists = decode_password_lists(std::include_bytes!(std::concat!(
        env!("OUT_DIR"),
        "/word_lists.bincode.deflate"
    )))
    .change_context(MainError::DecodeLists)?;

    let password_list = match cli.list {
        List::Short => &password_lists.short,
        List::Long => &password_lists.long,
        List::Memorable => &password_lists.memorable,
    };

    let password_list_len = u32::try_from(password_list.len())
        .change_context(MainError::Downcast)
        .attach_printable("password_list_len")?;

    let word_count = cli
        .count
        .unwrap_or_else(|| count_from_entropy(password_list_len, cli.entropy.unwrap_or(90)));

    let generated_password =
        diceware_password(password_list, word_count).change_context(MainError::Dice)?;

    let entropy = calculate_entropy(password_list_len, word_count)
        .change_context(MainError::EntropyCalculation)?;

    eprintln!("word count: {}", word_count);

    let entropy_bits_string = format!("{:.1} bits", entropy);
    let entropy_bits_string_colored = match entropy {
        ..70.0 => entropy_bits_string.red(),
        110.0.. => entropy_bits_string.green(),
        _ => entropy_bits_string.yellow(),
    };
    eprintln!("entropy: {}", entropy_bits_string_colored);

    if cli.copy_clipboard {
        copy_to_clipboard_and_delete(generated_password).change_context(MainError::Clipboard)?;
    } else {
        generated_password.str_scope(|s| println!("{}", s));
    }

    Ok(())
}
