use crate::err;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn eval_input(config_filepathbuf: &PathBuf) -> err::Feedback {
	let mut err_msg: String = String::new();

	if !config_filepathbuf.is_file() {
		writeln!(
			err_msg,
			"Expected config as file, but dir is provided: {}",
			config_filepathbuf.to_string_lossy()
		)
		.unwrap();
	}

	if err_msg.is_empty() {
		Ok(())
	} else {
		Err(err::Msg::from(err_msg))
	}
}

pub fn run(config_filepathbuf: PathBuf) {
	if let Err(err) = eval_input(&config_filepathbuf) {
		println!("{}", err);
		std::process::exit(1)
	}

	let config_dirpathbuf: &Path = config_filepathbuf.parent().expect("Expected a parent");

	fs::write(config_dirpathbuf.join("greetings.txt"), "Hello world!")
		.expect("Unable to write file");
}
