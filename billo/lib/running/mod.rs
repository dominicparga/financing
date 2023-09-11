use crate::err;
mod configuring;
use crate::running::configuring::{analysis, data};
use serde::de::DeserializeOwned;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct AnalysisConfig {
	pub filepath: PathBuf,
}

#[derive(Clone)]
pub struct DataConfig {
	pub filepath: PathBuf,
}

fn ensure_input_config<'a, CONFIG: DeserializeOwned>(
	config_filepathbuf: &PathBuf,
) -> err::Result<CONFIG> {
	let mut err_msg: String = String::new();

	if !config_filepathbuf.exists() {
		writeln!(
			err_msg,
			"Config file does not exist: {}",
			config_filepathbuf.to_string_lossy()
		)
		.expect("Should already handle error that config filepath being dirpath");
	} else if !config_filepathbuf.is_file() {
		writeln!(
			err_msg,
			"Expected config as file, but dir is provided: {}",
			config_filepathbuf.to_string_lossy()
		)
		.expect("Should already handle error that config filepath being dirpath");
	}

	if err_msg.is_empty() {
		let json_str: String = fs::read_to_string(config_filepathbuf)
			.map_err(|e| e.to_string())
			.map_err(err::Msg::from)?;
		Ok(serde_json::from_str(json_str.as_str())
			.map_err(|e| e.to_string())
			.map_err(err::Msg::from)?)
	} else {
		Err(err::Msg::from(err_msg))
	}
}

pub fn run(analysis_config: AnalysisConfig, data_config: DataConfig) {
	let analysis_config: analysis::Config = match ensure_input_config(&analysis_config.filepath) {
		Ok(config) => config,
		Err(err) => {
			println!("{}", err);
			std::process::exit(1)
		}
	};
	let data_config: data::Config = match ensure_input_config(&data_config.filepath) {
		Ok(config) => config,
		Err(err) => {
			println!("{}", err);
			std::process::exit(1)
		}
	};

	println!("{:?}", analysis_config);
	println!("{:?}", data_config);
}
