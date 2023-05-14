use billo;
use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
enum Command {
	/// Config based runner
	#[command(name = "run")]
	RunConfig {
		#[arg(long = "config", value_name = "PATH")]
		config_filepathbuf: PathBuf,
	},
}

#[derive(Parser)]
#[command(author)]
#[command(version)]
#[command(about = "This is BILLO at your service.")]
#[command(long_about=None)]
struct Cli {
	#[command(subcommand)]
	command: Command,
}

fn main() {
	let cli: Cli = Cli::parse();
	match cli.command {
		Command::RunConfig { config_filepathbuf } => billo::running::run(config_filepathbuf),
	}
}
