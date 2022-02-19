use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename())?;

    println!("Content of file: \n{}", contents);

    Ok(())
}

pub struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename})
    }

    pub fn query(&self) -> &str {
        &self.query
    }
    pub fn filename(&self) -> &str {
        &self.filename
    }
}