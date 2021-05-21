use std::error::Error;
use std::fs;
pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            file_name: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;
    println!("With text:\n{}", contents);
    Ok(())
}
