//! Kulupu CLI library.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;

pub use sc_cli::{VersionInfo, error};

fn main() -> Result<(), error::Error> {
	let version = VersionInfo {
		name: "Kulupu",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "kulupu",
		author: "Wei Tang <hi@that.world>",
		description: "Kulupu node implementation",
		support_url: "https://github.com/kulupu/kulupu/issues",
		copyright_start_year: 2019,
	};

	command::run(version)
}
