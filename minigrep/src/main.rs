use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments:{err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // Running the command below can potentially lead to error and we have not written any error handling code, so rust gives us a warning
    
    // run(config);

    // Thus we run the function through the error handling code below
    if let Err(e) = run(config) {
        println!("Application error:{e}");
        process::exit(1);
    }
}

// The run function's success type is () in the signature, which means we need to wrap
// the unit type value in the Ok value.

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
    // using () is the idiomatic way to indicate that we're calling run for its side
    // effects only, it doesn't return a value we need.
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// Box<dyn Error> means function will return a type that implements the Error trait.
// We don't need to specify what particular type the return value will be.
// dyn => dynamic
