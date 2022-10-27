use std::error::Error;
use std::fs;

mod input;

pub fn run(unparsed_args: impl Iterator<Item = String>) -> Result<(), String> {
    let args = parse_args(unparsed_args)?;

    let contents = match read_file(args.file_path()) {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    let result = if args.should_ignore_case() {
        search_case_insensitive(args.query(), &contents)
    } else {
        search_case_sensitive(args.query(), &contents)
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

fn parse_args(mut unparsed_args: impl Iterator<Item = String>) -> Result<input::Args, String> {
    unparsed_args.next();
    let query = match unparsed_args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a query string".to_owned()),
    };
    let file_path = match unparsed_args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a file path".to_owned()),
    };

    Ok(input::Args::new(file_path, query))
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    Ok(contents)
}

fn search<F>(contents: &str, f: F) -> Vec<&str>
where
    F: FnMut(&&str) -> bool,
{
    contents.lines().filter(f).collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    search(contents, |line| {
        line.to_lowercase().contains(&query.to_lowercase())
    })
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    search(contents, |line| line.contains(&query))
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

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        );
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
