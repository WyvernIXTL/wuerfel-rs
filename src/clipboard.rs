use std::{thread::sleep, time::Duration};

use colored::Colorize;
use copypasta::{ClipboardContext, ClipboardProvider};
use error_stack::{Report, report};
use thiserror::Error;
use zeroize::Zeroize;

#[derive(Debug, Clone, Error)]
pub enum CopyError {
    #[error("Failed to initialize the clipboard")]
    Init,
    #[error("Failed to write to clipboard")]
    Write,
    #[error("Failed to read clipboard")]
    Read,
}

pub fn copy_to_clipboard_and_delete(password: &str) -> Result<(), Report<CopyError>> {
    let mut ctx =
        ClipboardContext::new().map_err(|err| report!(CopyError::Init).attach_printable(err))?;

    let password_copy = password.to_owned();
    ctx.set_contents(password_copy)
        .map_err(|err| report!(CopyError::Write).attach_printable(err))?;

    eprintln!("{}", "clipboard is going to be deleted in 10s".yellow());

    sleep(Duration::from_secs(10));

    let mut cb_content = ctx
        .get_contents()
        .map_err(|err| report!(CopyError::Read).attach_printable(err))?;

    if cb_content == password {
        ctx.set_contents("".to_owned())
            .map_err(|err| report!(CopyError::Write).attach_printable(err))?;

        eprintln!("{}", "clipboard cleared!".green());
    } else {
        eprintln!("{}", "clipboard content changed, skipping cleanup".yellow())
    }

    cb_content.zeroize();

    Ok(())
}
