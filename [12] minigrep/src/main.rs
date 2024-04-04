use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problems parsing arguments: {err}");
        process::exit(1);
    });

    eprintln!("Searching for {}", config.query);
    eprintln!("in file {}", config.file_path);

    // Because we run doesn't return anything useful, we mainly called it for
    // its eprintln statement we can instead use it to handle
    // a possible returned Error
    if let Err(e) = minigrep::run(config) {
        eprintln!("Error reading the file: {e}");
        process::exit(1);
    }
}
