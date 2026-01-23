use crate::command::args_wrapper::ArgsWrapper;

pub struct ReplInput {
    pub identifier: String,
    pub args_wrapper: ArgsWrapper,
}

impl ReplInput {
    pub fn new(identifier: &str, argument: &[String]) -> Self {
        Self {
            identifier: String::from(identifier),
            args_wrapper: ArgsWrapper::new(argument.to_vec()),
        }
    }

    pub fn clone_identifier(&self) -> String {
        self.identifier.clone()
    }
}
