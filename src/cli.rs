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

use std::ffi::OsString;

use crate::{cliactions, utils};

use std::io::{Error, ErrorKind};

use clap::{AppSettings::ArgRequiredElseHelp, Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "gloves", version = "v0.1.0", author)]
/// Evm-based project environment.
///
/// This tools will help you for building ethereum
/// based contract project.
pub struct Cli {
    // commands for the app.
    #[clap(subcommand)]
    command: CliCommand,
}

// app commands.
#[derive(Subcommand)]
enum CliCommand {
    // command for initiating gloves project.
    #[clap(setting(ArgRequiredElseHelp))]
    /// Initiate new ethereum project.
    ///
    /// example: gloves init --compiler solcjs myproject.
    Init {
        /// path to current dir e.g `.` or to other dir e.g `./mytoken` or `mytoken`.
        path: String,
        #[clap(long = "compiler", default_value = "solc")]
        /// set the default solidity compiler binary e.g `solc`.
        compiler: String,
    },

    // command for creating new contract inside `contracts` dir.
    #[clap(setting(ArgRequiredElseHelp))]
    /// Create new `solidity` contract.
    ///
    /// example: gloves new MyToken.
    /// command above will create new file called `MyToken.sol` inside contracts directory.
    New {
        /// solidity contract name. e.g MyToken.
        contract: Vec<String>,
    },

    // command for compiling solidity contract inside `contracts` directory.
    #[clap()]
    /// Compile solidity contract.
    Compile {},

    // handling unknown command.
    #[clap(external_subcommand)]
    Ext(Vec<OsString>),
}

// run the cli app.
pub fn run() {
    let app = Cli::parse();

    match &app.command {
        CliCommand::Init { path, compiler } => {
            cliactions::init(path, compiler);
        }

        CliCommand::New { contract } => {
            cliactions::new(contract);
        }

        CliCommand::Compile {} => {
            cliactions::compile();
        }

        CliCommand::Ext(args) => {
            utils::errlog(
                format!("Error {:?}", args).as_str(),
                &Error::new(ErrorKind::NotFound, "command not found"),
            );
        }
    }
}
