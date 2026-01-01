pub struct ReplInput {
    pub identifier: String,
    pub argument: String,
}

impl ReplInput {
    pub fn new(identifier: &str, argument: &str) -> Self {
        Self {
            identifier: String::from(identifier),
            argument: String::from(argument),
        }
    }

    pub fn clone_identifier(&self) -> String {
        self.identifier.clone()
    }

    pub fn clone_argument_as_str(&self) -> String {
        self.argument.clone()
    }

    pub fn clone_argument_as_vec(&self) -> Vec<String> {
        self.argument.split_whitespace().map(String::from).collect()
    }
}
