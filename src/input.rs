use std::env;

pub struct Args {
    file_path: String,
    query: String,
    ignore_case: bool,
}

impl Args {
    pub fn new(file_path: String, query: String) -> Args {
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Args {
            file_path,
            query,
            ignore_case,
        }
    }

    pub fn file_path(&self) -> &String {
        &self.file_path
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn should_ignore_case(&self) -> bool {
        self.ignore_case
    }
}
