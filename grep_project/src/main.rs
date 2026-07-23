use std::env;
// this give access to cmd line arguments
// ex cargo run hello hello.txt
// ["program_name", "hello", "hello.txt"] first is always the name of the binary or the path to the program

// returns an iterators
// iterators are not vectors they give access to the values one at a time
// collect() method if you want all items at once

// Iterator  --collect()-->  Vector

// why not vectors immediately? because vectors are stored in memory and iterators are not. Iterators are lazy and only give access to the values one at a time.

use std::fs;

use ::grep_project::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!(
        "Searching for query: {} in file: {}",
        config.query, config.path_name
    );

    println!("In file: {}", config.path_name);

    let content = fs::read_to_string(config.path_name);
    // fs::read_to_string takes the file_path, opens that file, and returns a value of type std::io::Result<String> that contains the file’s contents.
    match content {
        Ok(content) => {
            println!("With text:\n{}", content);
            let matches = search(config.query, content.as_str());
            for line in matches {
                println!("{}", line);
            }
        }
        Err(_error) => {
            println!("Could not read file: {}", config.path_name);
        }
    }
}

fn parse_config<'a>(args: &'a [String]) -> Config<'a> {
    let config = Config::check_args(args);
    match config {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Problem parsing arguments: {}", err);
            std::process::exit(1);
        }
    }
}

use ::grep_project::search;
