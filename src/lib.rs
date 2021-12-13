use std::error::Error;
use std::fs;

// struct to hold parsed arguments
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // creates a new instance of Config thrugh vector of arguments (>=3 len)
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // println!("args: {:?}, len: {}", args, args.len()); // testing code
        // check that required arguments are supplied
        if args.len() < 3 { 
            return Err("not enough arguments");
        }
        let query = args[1].clone(); 
        let filename = args[2].clone(); 

        Ok(Config { query, filename }) 
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    Ok(())
}