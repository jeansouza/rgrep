use std::error::Error;
use std::fs;

mod input;

pub fn run(unparsed_args: Vec<String>) -> Result<(), String> {
    let args = parse_args(unparsed_args)?;

    let contents = match read_file(args.file_path()) {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    let result = if args.should_ignore_case() {
        search_case_insensitive(args.query(), &contents)
    } else {
        search(args.query(), &contents)
    };

    if result.is_empty() {
        println!("Nothing found");
        return Ok(());
    }

    for line in result {
        println!("{line}");
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

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    Ok(contents)
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
