// Copyright 2025 Adam McKellar
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bincode::Decode;
#[derive(Debug, Clone, Decode)]
pub struct PasswordLists {
    pub long: Vec<String>,
    pub short: Vec<String>,
    pub memorable: Vec<String>,
}
