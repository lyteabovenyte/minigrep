use std::env;
use std::process;
use minigrep::Config;
fn main() {
    let config = Config::build(&env::args().collect::<Vec<String>>())
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    } // run takes ownership of config.
}