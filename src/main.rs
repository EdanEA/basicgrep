use basicgrep::config::Config;
use std::{
    env,
    io::{self, Read},
    process,
};

fn main() {
    let stdin = io::stdin();
    let mut piped_in = false;
    let mut input = String::new();
    let args: Vec<String> = env::args().collect();

    if !atty::is(atty::Stream::Stdin) {
        let _ = match stdin.lock().read_to_string(&mut input) {
            Ok(_) => piped_in = true,
            Err(_) => {}
        };
    }

    let config = if !piped_in {
        Config::new(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        })
    } else {
        Config::new_from_pipe(&args, input.as_str()).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        })
    };

    // println!("{}", config);

    if let Err(e) = basicgrep::run(config) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
