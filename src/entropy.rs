// Copyright 2025 Adam McKellar
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use error_stack::{Report, ResultExt};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum EntropyCalculationError {
    #[error("Failed to downcast word count")]
    Downcast,
}

pub fn calculate_entropy(
    list_size: u32,
    word_count: u32,
) -> Result<f64, Report<EntropyCalculationError>> {
    let word_entropy: f64 = list_size.into();

    let entropy = word_entropy
        .powi(i32::try_from(word_count).change_context(EntropyCalculationError::Downcast)?);

    Ok(entropy.log2())
}

pub fn count_from_entropy(list_size: u32, entropy: u32) -> u32 {
    (f64::from(entropy) / f64::from(list_size).log2()).ceil() as u32
}
