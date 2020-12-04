use minigrep_v1::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("failed to load config, err: {:?}", err);
        process::exit(1);
    });

    if let Err(err) = Config::run(config) {
        eprintln!("app error: {:?}", err);
        process::exit(1);
    }
}
