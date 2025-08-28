// Copyright 2025 Adam McKellar
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use error_stack::{Report, ResultExt};
use rand::{SeedableRng, rngs::StdRng, seq::IndexedRandom};
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
}

pub fn diceware_password(
    list: &Vec<String>,
    length: u32,
) -> Result<SecureString, Report<DicewarePasswordGenError>> {
    let mut rng = StdRng::try_from_os_rng().change_context(DicewarePasswordGenError::SeedRng)?;

    let mut password =
        SecureString::new().change_context(DicewarePasswordGenError::SecureStringInit)?;

    for _ in 0..length - 1 {
        let word = list.choose(&mut rng).ok_or(DicewarePasswordGenError::Get)?;

        password.push_str(word);
        password.push_str(" ");
    }

    let word = list.choose(&mut rng).ok_or(DicewarePasswordGenError::Get)?;
    password.push_str(word);

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
