use error_stack::{Report, ResultExt};
use rand::{TryRngCore, rngs::OsRng};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum DiceError {
    #[error("Failed to get next random number")]
    Random,
}

pub fn dice(count_digits: u32) -> Result<u32, Report<DiceError>> {
    let mut result = 0;

    for _ in 0..count_digits {
        let digit = OsRng.try_next_u32().change_context(DiceError::Random)?;
        result = result * 10 + (digit % 6) + 1;
    }

    Ok(result)
}
