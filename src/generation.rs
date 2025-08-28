// Copyright 2025 Adam McKellar
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use error_stack::{Report, ResultExt};
use rand::{TryRngCore, rngs::OsRng};
use secure_types::SecureString;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum DicewarePasswordGenError {
    #[error("Failed to get random number from list")]
    Get,
    #[error("Failed to seed rng generator from os rng")]
    SeedRng,
    #[error("Failed to initialize a secure string")]
    SecureStringInit,
    #[error("Failed to downcast number")]
    Downcast,
}

pub fn diceware_password(
    list: &Vec<String>,
    length: u32,
) -> Result<SecureString, Report<DicewarePasswordGenError>> {
    let list_len = u64::try_from(list.len()).change_context(DicewarePasswordGenError::Downcast)?;

    let mut password =
        SecureString::new().change_context(DicewarePasswordGenError::SecureStringInit)?;

    for i in 0..length {
        let index = (OsRng
            .try_next_u64()
            .change_context(DicewarePasswordGenError::SeedRng)?
            % list_len) as usize;

        let word = list.get(index).ok_or(DicewarePasswordGenError::Get)?;

        password.push_str(word);

        if i != length - 1 {
            password.push_str(" ");
        }
    }

    Ok(password)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dice_ware_password() {
        let list = vec!["a".to_string(), "b".to_string()];

        let password = diceware_password(&list, 4).unwrap();

        assert_eq!(password.len(), 4 + 3);
    }
}
