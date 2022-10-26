use std::error::Error;
use std::fs;

mod input;

pub fn run(unparsed_args: Vec<String>) -> Result<(), String> {
    let args = parse_args(unparsed_args)?;
    if let Err(e) = read_file(args.file_path()) {
        return Err(e.to_string());
    }
    Ok(())
}

fn parse_args(mut unparsed_args: Vec<String>) -> Result<input::Args, String> {
    if unparsed_args.len() != 3 {
        return Err("Error! rgrep syntax is: rgrep <QUERY_STRING> <FILE_PATH>".to_owned());
    }
    let file_path = unparsed_args.remove(2);
    let query = unparsed_args.remove(1);

    Ok(input::Args::new(file_path, query))
}

fn read_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
