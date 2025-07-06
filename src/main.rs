use minigrep::Config;
use std::env;
use std::process;
fn main() {
    let config = Config::build(&env::args().collect::<Vec<String>>()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err); // standard error stream.
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    } // run takes ownership of config.
}
