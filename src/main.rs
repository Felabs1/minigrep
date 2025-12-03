use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

   // we store the query and filename in variables
   // note: args[0] is the name of the program itself... so we start at args[1]

   // use unwrap_or_else to handle the error cleanly
   let config = Config::build(&args).unwrap_or_else(|err| {
    println!("problem parsing arguments: {}", err);
    process::exit(1);
   });

    if let Err(e) = minigrep::run(config) {
     println!("Application error: {}", e);
     process::exit(1);
    }
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
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));

    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

}

