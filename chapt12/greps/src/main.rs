extern crate greps;

use std::env;
use std::process;

use greps::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);

    //     process::exit(1);
    // });
    let config: Config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = greps::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Config { query, filename }
// }
