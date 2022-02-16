use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();


    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problems parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query());
    println!("In file {}", config.filename());

    let contents = fs::read_to_string(config.filename())
        .expect("File read error");

    println!("Content of file: \n{}", contents)
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
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
