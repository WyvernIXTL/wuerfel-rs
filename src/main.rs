use clap::Parser;
use error_stack::{Report, ResultExt};
use license_fetcher::read_package_list_from_out_dir;
use thiserror::Error;

use crate::{
    cli::{Arguments, List},
    decode::decode_password_lists,
    generation::diceware_password,
};

mod cli;
mod decode;
mod generation;
mod password_lists;
mod random;

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
        List::Short => password_lists.short,
        List::Long => password_lists.long,
        List::Remember => password_lists.short_remember,
    };

    let digit_count: u32 = match cli.list {
        List::Short => 4,
        List::Long => 5,
        List::Remember => 4,
    };

    let generated_password =
        diceware_password(&password_list, digit_count, cli.count.unwrap_or(9).into())
            .change_context(MainError::Dice)?;

    println!("{}", generated_password);

    Ok(())
}
