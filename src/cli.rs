use structopt::StructOpt;
pub use sc_cli::RunCmd;

#[derive(Debug, StructOpt, Clone)]
pub enum Subcommand {
	#[structopt(flatten)]
	Base(sc_cli::Subcommand),

	#[structopt(name = "export-builtin-wasm", setting = structopt::clap::AppSettings::Hidden)]
	ExportBuiltinWasm(ExportBuiltinWasmCommand),
}

#[derive(Debug, StructOpt)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[structopt(flatten)]
	pub run: RunCmd,

	#[structopt(long)]
	pub author: Option<String>,
	#[structopt(long)]
	pub threads: Option<usize>,
	#[structopt(long)]
	pub round: Option<u32>,
}

#[derive(Debug, StructOpt, Clone)]
pub struct ExportBuiltinWasmCommand {
	#[structopt()]
	pub folder: String,
}
