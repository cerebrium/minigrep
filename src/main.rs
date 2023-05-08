use ::minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Use the unwrap_or_else since we need the return value
    // prefered to the match because it is less verbose
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // The return value is () so it is irelevant,
    // use the if let just to detect the error and
    // handle it.
    if let Err(err) = minigrep::run(config) {
        eprintln!("error in run: {}", err);
        process::exit(1);
    }
}
