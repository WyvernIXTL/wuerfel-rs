// Copyright 2025 Adam McKellar
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{ops::Deref, thread::sleep, time::Duration};

use colored::Colorize;
use copypasta::{ClipboardContext, ClipboardProvider};
use error_stack::{Report, report};
use secure_types::SecureString;
use thiserror::Error;
use zeroize::Zeroizing;

#[derive(Debug, Clone, Error)]
pub enum CopyError {
    #[error("Failed to initialize the clipboard")]
    Init,
    #[error("Failed to write to clipboard")]
    Write,
    #[error("Failed to read clipboard")]
    Read,
}

pub fn copy_to_clipboard_and_delete(password: SecureString) -> Result<(), Report<CopyError>> {
    let mut ctx =
        ClipboardContext::new().map_err(|err| report!(CopyError::Init).attach_printable(err))?;

    password.str_scope(|s| {
        ctx.set_contents(s.to_owned())
            .map_err(|err| report!(CopyError::Write).attach_printable(err))
    })?;

    eprintln!("{}", "clipboard is going to be deleted in 10s".yellow());

    sleep(Duration::from_secs(10));

    let cb_content = Zeroizing::new(
        ctx.get_contents()
            .map_err(|err| report!(CopyError::Read).attach_printable(err))?,
    );

    let password_in_cb = password.str_scope(|s| cb_content.deref() == s);

    if password_in_cb {
        ctx.set_contents("".to_owned())
            .map_err(|err| report!(CopyError::Write).attach_printable(err))?;

        eprintln!("{}", "clipboard cleared!".green());
    } else {
        eprintln!("{}", "clipboard content changed, skipping cleanup".yellow())
    }

    Ok(())
}
