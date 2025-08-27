use clap::{Parser, ValueEnum};

#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum List {
    /// Long word list
    ///
    /// Every word has ~ 13 bits of entropy.
    Long,
    /// Short word list
    ///
    /// Every word has ~ 10 bits of entropy.
    Short,
    /// Short rememberable word list
    ///
    /// Every word has ~ 10 bits of entropy.
    ///
    /// Used words are more common than either the words used in the long
    /// or the short password lists.
    Remember,
}

/// Diceware password generator cli based on eff password lists
///
/// The security of a password is measured in bits of entropy.
/// If you do not have control over the key derivation function (KDF)
/// used, I recommend entropy higher than 128 bits for encryption purposes and
/// an entropy higher than 80 bits for authentication purposes.
/// If you do have control over the KDF used, you may want to use more secure KDF parameters
/// to stretch the password used. This allows for less secure passwords, guaranteeing
/// a secure encryption.
#[derive(Debug, Clone, Parser)]
#[command(version, about)]
pub struct Arguments {
    /// Length of generated password in words
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..))]
    pub count: Option<u8>,
    /// List to use
    #[arg(short, long, value_enum, default_value_t = List::Short)]
    pub list: List,
    #[arg(long)]
    pub license: bool,
}
