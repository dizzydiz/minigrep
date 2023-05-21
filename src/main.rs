use std::env;
use std::fs;
use std::process;

fn main() {
    let args:Vec<String> = env::args().collect();
    // dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
    .expect("Should have been able to read the file");

    println!("With text:\n {contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'ststic str> {
        if args.len() != 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config{query, file_path})
    }
}

            
// fn parse_config(args: &Vec<String>) -> (&str, &str) {
//     let query = &args[1];
//     let file_path = &args[2];
    
//     (query, file_path)
// }