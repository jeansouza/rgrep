use std::env;
use std::process;

fn main() {
    rgrep::run(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
}
