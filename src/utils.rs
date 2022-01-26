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
    process,
};

use ansi_term::Color::{Green, Red, Yellow};

// generate absolute path from dynamic path.
pub fn path_absolute(path: &PathBuf) -> PathBuf {
    if path.is_relative() {
        match fs::canonicalize(path) {
            Ok(abs) => abs,
            Err(_) => path.to_path_buf(),
        }
    } else {
        path.to_path_buf()
    }
}

// checking existen of a directory.
pub fn dir_exist(path: &PathBuf) -> bool {
    let abs = path_absolute(path);

    abs.is_dir()
}

// checking existen of a file.
pub fn file_exist(path: &PathBuf) -> bool {
    let abs = path_absolute(path);

    abs.exists()
}

// create new directory.
pub fn dir_new(path: &PathBuf) {
    let abs = path_absolute(path);

    if dir_exist(&abs) {
        errlog(
            "Error creating new directory",
            &Error::new(ErrorKind::AlreadyExists, "Directory exist"),
        );
    }

    match fs::create_dir_all(abs) {
        Ok(_val) => oklog("Directory creation", "created"),
        Err(error) => errlog("Error creating directory", &error),
    }
}

// write new files.
pub fn file_new<C: AsRef<[u8]>>(path: &PathBuf, content: C) {
    let abs = path_absolute(path);

    match fs::write(abs, content) {
        Ok(_val) => oklog("Writing file", "writed"),
        Err(error) => errlog("Writing file", &error),
    }
}

// read a file.
pub fn file_read(path: PathBuf) -> String {
    let abs = path_absolute(&path);

    let mut res = fs::read_to_string(abs);

    if res.is_err() {
        errlog("Failed reading file", res.as_mut().err().unwrap());
    }

    res.unwrap().to_string()
}

// print an error statement.
pub fn errlog(title: &str, err: &std::io::Error) {
    let titletext = Red.bold().paint(title);
    let errtext = Red.paint(err.to_string());

    eprintln!("{} -> {}", titletext, errtext);
    process::exit(1);
}

// print an warning statement.
pub fn warnlog(warn: &str) {
    let title = Yellow.bold().paint("Warning");
    let warntext = Yellow.paint(warn);

    println!("{} -> {}", title, warntext);
}

// print an success statement.
pub fn oklog(title: &str, message: &str) {
    let titletext = Green.bold().paint(title);
    let messagetext = Green.paint(message);

    println!("{} -> {}", titletext, messagetext);
}
