use std::path::PathBuf;

use billo;
use billo::err;
use clap::Parser;
use clap::Subcommand;
use std::str::FromStr;

/// Sets the logging-level of this repo.
///
/// max_log_level: None
/// => use default (Warn)
///
/// modules: in addition to default (`env!("CARGO_PKG_NAME")`)
///
/// Environment-variable RUST_LOG has precedence.
pub fn init_logging(max_log_level: &str, modules: &[&str]) -> err::Feedback {
	let mut builder = env_logger::Builder::new();

	// maximum filter-level for all components: `warn`
	builder.filter(None, log::LevelFilter::Warn);

	// if quiet logging: doesn't log `info` for this repo
	let max_log_level = log::LevelFilter::from_str(&max_log_level.to_ascii_uppercase())
		.ok()
		.ok_or(format!(
			"The provided max-log-level {} is not supported.",
			max_log_level
		))?;
	for module in modules {
		builder.filter(Some(module), max_log_level);
	}
	builder.filter(Some(env!("CARGO_PKG_NAME")), max_log_level);

	// overwrite default with environment-variables
	if let Ok(filters) = std::env::var("RUST_LOG") {
		builder.parse_filters(&filters);
	}
	if let Ok(write_style) = std::env::var("RUST_LOG_STYLE") {
		builder.parse_write_style(&write_style);
	}

	// init
	builder.init();

	// return
	Ok(())
}

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
	init_logging("INFO", &[]).expect("LogLevel 'INFO' does exist.");

	let cli: Cli = Cli::parse();
	match cli.command {
		Command::RunConfig {
			analysis_config,
			data_config,
		} => billo::running::run(
			billo::running::AnalysisConfigFilepath(analysis_config),
			billo::running::DataConfigFilepath(data_config),
		),
	}
}
