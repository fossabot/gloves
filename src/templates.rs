// Copyright (C) 2022  The gloves Authors.
// This file is part of gloves.
//
// gloves is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// gloves is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with gloves.  If not, see <https://www.gnu.org/licenses/>.

use serde::{Deserialize, Serialize};
use toml;

// config templates.
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub info: ConfigInfo,
    pub compiler: ConfigCompiler,
}

// config.info templates.
#[derive(Deserialize, Serialize)]
pub struct ConfigInfo {
    pub name: String,
    pub license: String,
}

// config.compiler templates.
#[derive(Deserialize, Serialize)]
pub struct ConfigCompiler {
    pub solc: String,
    pub optimize: bool,
    pub runs: i64,
}

// generate new Config.
pub fn config_new(name: &str, compiler: &str, license: &str) -> String {
    let config = Config {
        info: ConfigInfo {
            name: name.to_string(),
            license: license.to_uppercase().to_string(),
        },
        compiler: ConfigCompiler {
            solc: compiler.to_string(),
            optimize: false,
            runs: 0,
        },
    };

    toml::to_string_pretty(&config).unwrap()
}

// decode config file.
pub fn config_decode(config_string: &str) -> Config {
    let config: Config = toml::from_str(config_string).unwrap();

    config
}

// generate solidity templates.
pub fn solidity_new(name: &str, license: &str) -> String {
    format!(
        "// SPDX-License-Identifier: {}

pragma solidity ^0.8.10;

contract {} {{
    constructor() {{}}
}}",
        license, name
    )
}
