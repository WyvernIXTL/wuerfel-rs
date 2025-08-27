// Copyright 2025 Adam McKellar
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use error_stack::{Report, ResultExt, ensure};
use rand::{TryRngCore, rngs::OsRng};
use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq)]
pub enum DiceError {
    #[error("Failed to get next random number from OS")]
    Random,
    #[error("Requested digit count is invalid")]
    InvalidCount,
}

pub fn dice(count_digits: u32) -> Result<u32, Report<DiceError>> {
    ensure!(count_digits > 0, DiceError::InvalidCount);

    let mut result = 0;

    for _ in 0..count_digits {
        let digit = OsRng.try_next_u32().change_context(DiceError::Random)?;
        result = result * 10 + (digit % 6) + 1;
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::random::{DiceError, dice};

    #[test]
    fn test_dice_0_digits() {
        let result = dice(0);
        assert_eq!(
            *result.unwrap_err().current_context(),
            DiceError::InvalidCount
        );
    }

    #[test]
    fn test_dice_1_digit() {
        let result = dice(1).unwrap();
        assert!(result >= 1);
        assert!(result <= 6);
    }

    #[test]
    fn test_dice_2_digit() {
        let result = dice(2).unwrap();

        let first_digit = result / 10;
        assert!(first_digit >= 1);
        assert!(first_digit <= 6);

        let second_digit = result % 10;
        assert!(second_digit >= 1);
        assert!(second_digit <= 6);
    }
}
