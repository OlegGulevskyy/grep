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
	let res = search(&config.query, &content);

	println!("Search result: {:?}", res);

	Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	let results: Vec<&str> = content.lines().filter(|val| {
		val.contains(query)
	}).collect();

	results
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one_result() {
		let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents))
	}
}