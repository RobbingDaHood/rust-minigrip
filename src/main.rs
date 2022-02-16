use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("query: {}", query);
    println!("filename: {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("File read error");

    println!("Content of file: \n{}", contents)
}
