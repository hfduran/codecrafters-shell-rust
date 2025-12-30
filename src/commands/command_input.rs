pub struct CommandInput {
    pub identifier: String,
    pub argument: String
}

impl CommandInput {
    pub fn new(identifier: &str, argument: &str) -> Self {
        Self {
            identifier : String::from(identifier),
            argument: String::from(argument)
        }
    }

    pub fn clone_identifier(&self) -> String {
        self.identifier.clone()
    }

    pub fn clone_argument(&self) -> String {
        self.argument.clone()
    }
}