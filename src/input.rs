pub struct Args {
    file_path: String,
    query: String,
}

impl Args {
    pub fn new(file_path: String, query: String) -> Args {
        Args { file_path, query }
    }

    pub fn file_path(&self) -> &String {
        &self.file_path
    }

    pub fn query(&self) -> &String {
        &self.query
    }
}
