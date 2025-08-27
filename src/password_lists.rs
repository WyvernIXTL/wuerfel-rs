use std::collections::HashMap;

use bincode::{Decode, Encode};

pub type PasswordList = HashMap<u32, String>;

#[derive(Debug, Clone, Encode, Decode)]
pub struct PasswordLists {
    pub long: PasswordList,
    pub short: PasswordList,
    pub short_remember: PasswordList,
}
