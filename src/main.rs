use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    rgrep::run(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
}
