use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,  
}

impl Config {
    // a constructor to parse the args
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path } )
    }
}

// Box<dyn Error> allows us to return any type of error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // iterate over the results and print lines that match
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}