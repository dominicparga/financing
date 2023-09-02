use crate::err;
mod datastructures;
use crate::running::datastructures::Config;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

fn ensure_input(config_filepathbuf: &PathBuf) -> err::Result<Config> {
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
		Ok(serde_json::from_str(&json_str)
			.map_err(|e| e.to_string())
			.map_err(err::Msg::from)?)
	} else {
		Err(err::Msg::from(err_msg))
	}
}

pub fn run(config_filepathbuf: PathBuf) {
	let config: Config = match ensure_input(&config_filepathbuf) {
		Ok(config) => config,
		Err(err) => {
			println!("{}", err);
			std::process::exit(1)
		}
	};

	println!("{:?}", config)
}
