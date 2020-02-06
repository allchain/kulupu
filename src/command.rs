// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

use std::{path::PathBuf, fs::File, io::Write};
use log::info;
use sc_cli::{VersionInfo, error};
use crate::service;
use crate::chain_spec;
use crate::cli::{Cli, Subcommand};

/// Parse and run command line arguments
pub fn run(version: VersionInfo) -> error::Result<()>
{
	let opt = sc_cli::from_args::<Cli>(&version);

	let mut config = sc_service::Configuration::default();
	config.impl_name = "kulupu-node";

	match opt.subcommand {
		Some(Subcommand::Base(subcommand)) => sc_cli::run_subcommand(
			config,
			subcommand,
			chain_spec::load_spec,
			|config| Ok(new_full_start!(config, None).0),
			&version,
		),
		Some(Subcommand::ExportBuiltinWasm(cmd)) => {
			info!("Exporting builtin wasm binary to folder: {}", cmd.folder);
			let folder = PathBuf::from(cmd.folder);

			{
				let mut path = folder.clone();
				path.push("kulupu_runtime.compact.wasm");
				let mut file = File::create(path)?;
				file.write_all(&kulupu_runtime::WASM_BINARY)?;
				file.flush()?;
			}

			{
				let mut path = folder.clone();
				path.push("kulupu_runtime.wasm");
				let mut file = File::create(path)?;
				file.write_all(&kulupu_runtime::WASM_BINARY_BLOATY)?;
				file.flush()?;
			}

			Ok(())
		},
		None => sc_cli::run(
			config,
			opt.run.clone(),
			|config| service::new_light(
				config,
				opt.author.as_ref().map(|s| s.as_str())
			),
			|config| service::new_full(
				config,
				opt.author.as_ref().map(|s| s.as_str()),
				opt.threads.unwrap_or(1),
				opt.round.unwrap_or(5000),
			),
			chain_spec::load_spec,
			&version,
		)
	}
}
