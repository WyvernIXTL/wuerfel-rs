use bincode::config::Configuration;
use error_stack::{Report, ResultExt, ensure};
use thiserror::Error;

use crate::password_lists::PasswordLists;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Failed to decompress password lists")]
    Decompress,
    #[error("Failed to decode password lists")]
    Decode,
    #[error("Nothing was decoded")]
    Empty,
}

pub static BINCODE_CONFIG: Configuration = bincode::config::standard();

pub fn decode_password_lists(bin: &[u8]) -> Result<PasswordLists, Report<DecodeError>> {
    let uncompressed =
        miniz_oxide::inflate::decompress_to_vec(bin).change_context(DecodeError::Decompress)?;

    let (decoded, size): (PasswordLists, usize) =
        bincode::decode_from_slice(&uncompressed, BINCODE_CONFIG)
            .change_context(DecodeError::Decode)?;

    ensure!(size > 0, DecodeError::Empty);

    Ok(decoded)
}
