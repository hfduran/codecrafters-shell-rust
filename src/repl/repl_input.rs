use crate::command::args_wrapper::ArgsWrapper;

pub struct ReplInput {
    pub identifier: String,
    pub args_wrapper: ArgsWrapper,
}

impl ReplInput {
    pub fn new(identifier: &str, argument: &str) -> Self {
        Self {
            identifier: String::from(identifier),
            args_wrapper: ArgsWrapper::new(argument),
        }
    }

    pub fn clone_identifier(&self) -> String {
        self.identifier.clone()
    }
}
