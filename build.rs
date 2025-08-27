// Copyright 2025 Adam McKellar
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use std::env::var_os;
use std::fs::{read_to_string, write};
use std::sync::LazyLock;

use bincode::config::Configuration;
use license_fetcher::build::config::{Config, ConfigBuilder};
use license_fetcher::build::package_list_with_licenses;
use license_fetcher::{PackageList, package};

use bincode::{Decode, Encode};
use regex_lite::{Regex, RegexBuilder};

type PasswordList = HashMap<u32, String>;

#[derive(Debug, Clone, Encode, Decode)]
struct PasswordLists {
    pub long: PasswordList,
    pub short: PasswordList,
    pub short_remember: PasswordList,
}

fn package_licenses() {
    let config: Config = ConfigBuilder::from_build_env()
        .build()
        .expect("Failed to build configuration.");

    let mut packages: PackageList =
        package_list_with_licenses(config).expect("Failed to fetch metadata or licenses.");

    packages.push(package! {
        name: "EFF Wordlists".to_string(),
        version: "".to_string(),
        authors: vec!["Joseph Bonneau".to_string()],
        description: None,
        homepage: Some("https://www.eff.org/dice".to_string()),
        repository: None,
        license_identifier: Some("CC-BY-1.0".to_string()),
        license_text: Some(read_to_string("./password-lists/LICENSE").expect("Failed to read license of eff wordlists"))
    });

    packages
        .write_package_list_to_out_dir()
        .expect("Failed to write package list.");
}

fn parse_password_list(list: String) -> PasswordList {
    static WORDLIST_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        RegexBuilder::new(r"(?m)^(?<number>[1-6]{4,5})\s?(?<word>[\w\-]*)$")
            .crlf(true)
            .build()
            .expect("Failed to parse regex")
    });

    let mut parsed_list: PasswordList = HashMap::new();

    for (_, [number, word]) in WORDLIST_REGEX
        .captures_iter(&list)
        .map(|capture| capture.extract())
    {
        let number_parsed =
            u32::from_str_radix(number, 10).expect("Failed to parse number of Wordlist");

        parsed_list.insert(number_parsed, word.into());
    }

    parsed_list
}

static BINCODE_CONFIG: Configuration = bincode::config::standard();

fn encode_password_lists(lists: PasswordLists) -> Vec<u8> {
    let encoded =
        bincode::encode_to_vec(lists, BINCODE_CONFIG).expect("Failed to encode password lists");

    miniz_oxide::deflate::compress_to_vec(&encoded, 10)
}

fn package_password_lists() {
    let lists = PasswordLists {
        long: parse_password_list(
            read_to_string("./password-lists/eff_long_passwordlist.txt")
                .expect("Failed to read password list file"),
        ),
        short: parse_password_list(
            read_to_string("./password-lists/eff_short_passwordlist.txt")
                .expect("Failed to read password list file"),
        ),
        short_remember: parse_password_list(
            read_to_string("./password-lists/eff_short_passwordlist_remember.txt")
                .expect("Failed to read password list file"),
        ),
    };

    let encoded_lists = encode_password_lists(lists);

    let mut path = var_os("OUT_DIR").expect("Failed to fetch out dir environment variable");
    path.push("/word_lists.bincode.deflate");

    write(path, encoded_lists)
        .expect("Failed to write encoded password lists to temporary compilation folder");
}

fn main() {
    package_licenses();

    package_password_lists();

    // Rerun only if one of the following files changed:
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=Cargo.lock");
    println!("cargo::rerun-if-changed=Cargo.toml");
}
