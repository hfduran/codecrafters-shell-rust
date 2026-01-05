#[derive(Clone)]
pub struct ArgsWrapper {
    raw_string: String,
}

impl ArgsWrapper {
    pub fn new(raw_string: &str) -> Self {
        Self {
            raw_string: String::from(raw_string),
        }
    }

    pub fn get_args_vec(&self) -> Vec<String> {
        self.raw_string
            .split_whitespace()
            .map(String::from)
            .collect()
    }
}
