use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // create a new vector from the arguments 
    let args: Vec<String> = env::args().collect();  

    // call the Config struct constructor
    let config = Config::new(&args).unwrap_or_else(|err| { 
        println!("Problem parsing arguments: {}", err); 
        process::exit(1);
    });

    // execute the run() func which holds all the logic
    // to search for query in file
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }    
}
