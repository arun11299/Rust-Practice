use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub struct Config
{
    pub query : String,
    pub filename : String,
}

impl Config {
    // Create a new configuration from the args
    pub fn new(args : &Vec<String>) -> Result<Config, &'static str>
    {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config{
            query : args[1].clone(),
            filename : args[2].clone(),
        })
    }
}

///
pub fn run(cfg : &Config) -> Result<(), Box<Error>>
{
    let mut fd = File::open(&cfg.filename)?;
    let mut contents = String::new();

    fd.read_to_string(&mut contents)?;

    for line in search(cfg.query.as_str(), &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query : &str, contents : &'a str) -> Vec<&'a str>
{
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
