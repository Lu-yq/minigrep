use std::env;
use std::process;

use minigrep::{Config, run};

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // let config = parse_config(&args);
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}

// fn parse_config(args: &Vec<String>) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config {query, file_path}
    // Config::new(args)
    // Config::build(&args[..])
// }
