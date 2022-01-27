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
    fs,
    io::{Error, ErrorKind},
    path::PathBuf,
};

use crate::{solc, templates, utils};

// init action for initiating gloves project.
pub fn init(path: &String, compiler: &String, license: &String) {
    // path variable
    let dir = PathBuf::from(path);
    let abs_dir = utils::path_absolute(&dir);
    let mut cfg_file_path = PathBuf::from(path);
    cfg_file_path.push("gloves.toml");
    // error variable
    let err_initiated = Error::new(ErrorKind::AlreadyExists, "directory not empty.");
    let err_solc_check = Error::new(ErrorKind::Other, "compiler doesn't exist.");

    // begin actions

    // checking directory
    if !utils::dir_exist(&dir) {
        utils::dir_new(&dir);
    }

    // checking compiler
    if !solc::solc_exist(compiler) {
        utils::errlog("Error checking compiler", &err_solc_check);
    }

    match solc::solc_version(compiler) {
        Ok(res) => utils::oklog(
            format!("Using {}", compiler.as_str()).as_str(),
            res.as_str(),
        ),
        Err(_) => {
            let err = Error::new(ErrorKind::Other, "Error getting compiler version");
            utils::errlog("Getting compiler version", &err);
        }
    }

    // checking config file
    if utils::file_exist(&cfg_file_path) {
        utils::errlog("Failed initiating project", &err_initiated);
    }

    let config = templates::config_new(
        abs_dir.file_name().unwrap().to_str().unwrap(),
        compiler,
        license,
    );
    utils::file_new(&cfg_file_path, config);

    // finish
    utils::oklog(
        "Project created",
        abs_dir.file_name().unwrap().to_owned().to_str().unwrap(),
    );
}

// new action for creating new solidity contract.
pub fn new(contract: &Vec<String>) {
    // path variable
    let contract_dir = PathBuf::from("./contracts");
    let cfg_file_path = PathBuf::from("./gloves.toml");

    // check if current dir is gloves project
    if !utils::file_exist(&cfg_file_path) {
        let err = Error::new(ErrorKind::NotFound, "gloves config not found.");

        utils::errlog("Error creating new contract", &err);
    }

    // get the config file.
    let cfg_string = utils::file_read(cfg_file_path);
    let cfg_content = templates::config_decode(cfg_string.as_str());

    // check contract dir
    if !utils::dir_exist(&contract_dir) {
        utils::dir_new(&contract_dir);
    }

    // begin actions
    for ct in contract {
        let mut ct_file = PathBuf::from(&contract_dir);
        let content = templates::solidity_new(ct, cfg_content.info.license.as_str());
        ct_file.push(format!("{}.sol", ct));
        utils::file_new(&ct_file, content.as_str());

        utils::oklog(ct_file.file_name().unwrap().to_str().unwrap(), "created");
    }
}

// compile action for compiling solidity contract.
pub fn compile() {
    // path variable
    let contract_dir = PathBuf::from("./contracts");
    let cfg_file_path = PathBuf::from("./gloves.toml");

    // check if current dir is gloves project
    if !utils::file_exist(&cfg_file_path) {
        let err = Error::new(ErrorKind::NotFound, "gloves config not found.");

        utils::errlog("Error compiling contract", &err);
    }

    // get the config file
    let cfg_string = utils::file_read(cfg_file_path);

    // begin actions

    // check contracts directory
    if !utils::dir_exist(&contract_dir) {
        let err = Error::new(ErrorKind::NotFound, "./contracts directory not found");
        utils::errlog("Error compiling contract", &err);
    }

    let dir_contents = fs::read_dir(&contract_dir).unwrap();
    let dir_count = fs::read_dir(&contract_dir).unwrap().count();

    if dir_count < 1 {
        let err = Error::new(ErrorKind::NotFound, "Empty directory");

        utils::errlog("Error compiling contract", &err);
    }

    for f in dir_contents {
        let f_path = f.unwrap().path();
        if !f_path.is_dir() {
            if f_path.to_str().unwrap().contains(".sol") {
                solc::solc_compile(
                    &templates::config_decode(&cfg_string).compiler.solc,
                    f_path.to_str().unwrap(),
                    "./artifacts",
                    templates::config_decode(&cfg_string),
                );
            }
        }
    }
}
