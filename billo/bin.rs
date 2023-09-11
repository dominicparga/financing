use std::path::PathBuf;

use billo;
use clap::Parser;
use clap::Subcommand;

#[derive(Subcommand)]
enum Command {
	/// Config based runner
	#[command(name = "run")]
	RunConfig {
		#[arg(long = "analysis", value_name = "PATH")]
		analysis_config: PathBuf,
		#[arg(long = "data", value_name = "PATH")]
		data_config: PathBuf,
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
		Command::RunConfig {
			analysis_config,
			data_config,
		} => billo::running::run(
			billo::running::AnalysisConfig {
				filepath: analysis_config,
			},
			billo::running::DataConfig {
				filepath: data_config,
			},
		),
	}
}
