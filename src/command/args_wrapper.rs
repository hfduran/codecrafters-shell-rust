#[derive(Clone)]
pub struct ArgsWrapper {
    args: Vec<String>,
}

impl ArgsWrapper {
    pub fn new(args: Vec<String>) -> Self {
        Self {
            args: args
        }
    }

    pub fn get_args_vec(&self) -> Vec<String> {
        self.args.clone()
    }
}
