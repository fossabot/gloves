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

pub mod cli;
pub mod cliactions;
pub mod solc;
pub mod templates;
pub mod utils;

#[cfg(test)]
mod test {
    use std::{env::temp_dir, fs, path::PathBuf};

    use crate::utils;

    #[test]
    fn test_dir_exist() {
        let exist_dir = PathBuf::from("./");

        assert_eq!(utils::dir_exist(&exist_dir), true);

        let non_exist_dir = PathBuf::from("./non_exist");

        assert_ne!(utils::dir_exist(&non_exist_dir), true);
    }

    #[test]
    fn test_file_exist() {
        let exist_file = PathBuf::from("./Cargo.toml");

        assert_eq!(utils::file_exist(&exist_file), true);

        let non_exist_file = PathBuf::from("./non_exist_file.odgj");

        assert_ne!(utils::file_exist(&non_exist_file), true);
    }

    #[test]
    fn test_dir_new() {
        let dir = PathBuf::from("./test_dir");
        utils::dir_new(&dir);

        assert_eq!(utils::dir_exist(&dir), true);

        match fs::remove_dir(dir) {
            Ok(_) => return,
            Err(err) => panic!("{:?}", err),
        }
    }

    #[test]
    fn test_file_new_and_file_read() {
        let dir = temp_dir();
        let mut file = PathBuf::from(&dir);
        file.push("test.txt");

        utils::file_new(&file, "test");

        assert_eq!(utils::file_exist(&file), true);

        assert_eq!(utils::file_read(file), "test");

        drop(dir);
    }

    #[test]
    fn test_path_absolute() {
        let dir = PathBuf::from("./");
        let abs = utils::path_absolute(&dir);

        assert_eq!(abs.is_absolute(), true);
        assert_ne!(abs.is_relative(), true);
    }
}
