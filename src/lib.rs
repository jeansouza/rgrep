use std::error::Error;
use std::fs;

mod input;

pub fn run(unparsed_args: Vec<String>) -> Result<(), &'static str> {
    let args = parse_args(unparsed_args)?;
    read_file(args.file_path())?;
    Ok(())
}

fn parse_args(mut unparsed_args: Vec<String>) -> Result<input::Args, &'static str> {
    if unparsed_args.len() != 3 {
        return Err("Error! rgrep syntax is: rgrep <QUERY_STRING> <FILE_PATH>");
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
