// Copyright 2025 Adam McKellar
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use error_stack::{Report, ResultExt};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum EntropyCalculationError {
    #[error("Overflow during multiplication")]
    Overflow,
    #[error("Failed to downcast word count")]
    Downcast,
}

pub fn calculate_entropy(
    digit_count: u32,
    word_count: u32,
) -> Result<f64, Report<EntropyCalculationError>> {
    let word_entropy = 6u32
        .checked_pow(digit_count)
        .ok_or(EntropyCalculationError::Overflow)?;

    let word_entropy_float: f64 = word_entropy.into();
    let entropy = word_entropy_float
        .powi(i32::try_from(word_count).change_context(EntropyCalculationError::Downcast)?);

    Ok(entropy.log2())
}

pub fn count_from_entropy(digit_count: u32, entropy: u32) -> u32 {
    // entropy / 6u32.pow(digit_count).ilog2()
    (f64::from(entropy) / 6f64.powf(f64::from(digit_count)).log2()).ceil() as u32
}
