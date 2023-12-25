use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err: &str| {
        eprintln!("Error : problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("-> searching for \"{}\"", config.query);
    println!("-> in path {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Error : application error: {}", e);
        process::exit(1);
    }
}
