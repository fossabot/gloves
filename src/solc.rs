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

use std::{
    io::{Error, ErrorKind},
    process::Command,
};

use crate::{templates, utils};

// check if compiler exist.
pub fn solc_exist(compiler: &str) -> bool {
    let cmd = Command::new(compiler).arg("--version").output();

    if cmd.is_err() {
        return false;
    }

    return true;
}

// check compiler version.
pub fn solc_version(compiler: &str) -> Result<String, bool> {
    match Command::new(compiler).arg("--version").output() {
        Ok(res) => Ok(unsafe {
            let long = std::str::from_utf8_unchecked(&res.stdout).to_string();

            long.lines().last().unwrap().to_string()
        }),
        Err(_err) => Err(false),
    }
}

// compile solidity source file.
pub fn solc_compile(compiler: &str, file: &str, out: &str, config: templates::Config) {
    let mut args = vec![
        "--bin",
        "--abi",
        "--base-path",
        "./contracts",
        "--include-path",
        "./libs",
        "--include-path",
        "./node_modules",
        "--output-dir",
        out,
    ];

    let runs: String;
    if config.compiler.optimize {
        runs = config.compiler.runs.to_string();
        args.push("--optimize");
        args.push("--optimize-runs");
        args.push(runs.as_str());
    }

    args.push(file);

    let cmd = Command::new(compiler).args(args).output();

    // cmd error handler.
    if cmd.is_err() {
        let err = Error::new(ErrorKind::Interrupted, cmd.err().unwrap());

        utils::errlog("Error compiling contracts", &err);
    } else {
        let ok = cmd.ok().unwrap();
        // check error on stderr
        if ok.stderr.len() > 0 {
            let err_str = unsafe { std::str::from_utf8_unchecked(&ok.stderr) };
            // if error is present throw
            if err_str.contains("Error") || err_str.contains("error") {
                let err = Error::new(ErrorKind::Interrupted, err_str);

                utils::errlog("Error compiling contracts", &err);
            } else {
                utils::warnlog(err_str);
            }
        }

        println!("{:?}", ok);
        // show success
        utils::oklog("Contract compiled", file);
    }
}
