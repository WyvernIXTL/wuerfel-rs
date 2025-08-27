// Copyright 2025 Adam McKellar
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use bincode::{Decode, Encode};

pub type PasswordList = HashMap<u32, String>;

#[derive(Debug, Clone, Encode, Decode)]
pub struct PasswordLists {
    pub long: PasswordList,
    pub short: PasswordList,
    pub short_remember: PasswordList,
}
