use error_stack::{Report, ResultExt};
use license_fetcher::read_package_list_from_out_dir;
use thiserror::Error;

use crate::{decode::decode_password_lists, random::dice};

mod decode;
mod password_lists;
mod random;

#[derive(Debug, Clone, Error)]
enum MainError {
    #[error("Failed to decode password lists")]
    DecodeLists,
    #[error("Failed to dice")]
    Dice,
}

fn main() -> Result<(), Report<MainError>> {
    println!("Hello, world!");

    let package_list = read_package_list_from_out_dir!().unwrap();

    // println!("{}", package_list);

    let password_lists = decode_password_lists(std::include_bytes!(std::concat!(
        env!("OUT_DIR"),
        "/word_lists.bincode.deflate"
    )))
    .change_context(MainError::DecodeLists)?;

    dbg!(password_lists.long.len());
    dbg!(password_lists.short.len());
    dbg!(password_lists.short_remember.len());

    dbg!(dice(5).change_context(MainError::Dice)?);
    dbg!(dice(5).change_context(MainError::Dice)?);
    dbg!(dice(4).change_context(MainError::Dice)?);
    dbg!(dice(5).change_context(MainError::Dice)?);
    dbg!(dice(5).change_context(MainError::Dice)?);

    Ok(())
}
