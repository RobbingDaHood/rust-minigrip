use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrip::Config::new(&args).unwrap_or_else(|err| {
        println!("Problems parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.get_query());
    println!("In file {}", config.get_filename());

    if let Err(e) = minigrip::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
