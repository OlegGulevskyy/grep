use std::fs;
use std::error::Error;
pub struct Config {
	pub query: String,
	pub filename: String,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &str> {
		if args.len() < 3 {
			return Err("Not enough arguments")
		}

		let query = &args[1].clone();
		let file_name = &args[2].clone();
	
		Ok(Config { query: query.to_string(), filename: file_name.to_string() })
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let content = fs::read_to_string(config.filename)?;

	println!("Found content: {}", content);

	Ok(())
}
