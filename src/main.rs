use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Read the argument values
    // let args: Vec<String> = env::args().collect();
    // dbg!(args); // debug macro

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
