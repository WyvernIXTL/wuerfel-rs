use error_stack::{Report, ResultExt};
use thiserror::Error;

use crate::{password_lists::PasswordList, random::dice};

#[derive(Debug, Clone, Error)]
pub enum DicewarePasswordGenError {
    #[error("Failed to generate a random number")]
    Dice,
    #[error("Failed to get random number from list")]
    Get,
}

pub fn diceware_password(
    list: &PasswordList,
    digit_count: u32,
    length: u32,
) -> Result<String, Report<DicewarePasswordGenError>> {
    let mut password = String::new();

    for _ in 0..length {
        let random_number = dice(digit_count).change_context(DicewarePasswordGenError::Dice)?;
        let word = list
            .get(&random_number)
            .ok_or(DicewarePasswordGenError::Get)
            .attach_printable_lazy(|| format!("random number: {}", random_number))?;
        password.push_str(word);
        password.push(' ');
    }

    Ok(password.trim_end().to_owned())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dice_ware_password() {
        let list: PasswordList = vec![
            (1, "a".to_owned()),
            (2, "b".to_owned()),
            (3, "c".to_owned()),
            (4, "d".to_owned()),
            (5, "e".to_owned()),
            (6, "f".to_owned()),
        ]
        .into_iter()
        .collect();

        let password = diceware_password(&list, 1, 4).unwrap();

        assert_eq!(password.len(), 4 + 3);
    }
}
