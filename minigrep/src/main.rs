use std::env;
use std::fs::File;
use std::process;

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn main() {
    // essa funcao nao aceita unicode
    let args: Vec<String> = env::args().collect();

    if let Err(e) = run(config) {
        println!("application error: {}", e);
        process::exit(1);
    }

    println!("searching for '{}'", config.query);
    println!("in file {}", config.filename);

    let mut _f = File::open(config.filename).expect("file not found");
}

struct Config {
    query: String,
    filename: String,
}
