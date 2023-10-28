use crate::err;
mod computing;
mod configuring;
use crate::running::configuring::{AnalysisConfig, DataConfig};
use log;
use serde::de::DeserializeOwned;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct AnalysisConfigFilepath(pub PathBuf);

#[derive(Clone)]
pub struct DataConfigFilepath(pub PathBuf);

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

pub fn run(
	analysis_config_filepath: AnalysisConfigFilepath,
	data_config_filepath: DataConfigFilepath,
) {
	log::info!(
		"Parsing analysis config {}",
		analysis_config_filepath.0.to_string_lossy()
	);
	let analysis_config: AnalysisConfig = match ensure_input_config(&analysis_config_filepath.0) {
		Ok(config) => config,
		Err(err) => {
			println!("{}", err);
			std::process::exit(1);
		}
	};
	log::info!(
		"Parsing data config {}",
		data_config_filepath.0.to_string_lossy()
	);
	let data_config: DataConfig = match ensure_input_config(&data_config_filepath.0) {
		Ok(config) => config,
		Err(err) => {
			log::error!("{}", err);
			std::process::exit(1)
		}
	};

	for analysis in &analysis_config.analysis_list {
		match analysis {
			configuring::Analysis::Sum {
				headline,
				label_list,
				time_period,
			} => {
				let result_list = computing::compute_sums(&label_list, *time_period, &data_config);
				println!("{}", headline);
				println!("{:#?}", result_list);
				println!();
			}
		};
	}
}
