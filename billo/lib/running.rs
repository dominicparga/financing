use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn run(config_filepathbuf: PathBuf) {
	let config_dirpathbuf: &Path = {
		if config_filepathbuf.is_file() {
			config_filepathbuf.parent().expect("Expected a parent")
		} else {
			&config_filepathbuf
		}
	};

	fs::write(config_dirpathbuf.join("greetings.txt"), "Hello world!")
		.expect("Unable to write file");
}
