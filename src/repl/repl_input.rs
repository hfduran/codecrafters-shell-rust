pub struct ReplInput {
    pub program: String,
    pub args: Vec<String>,
}

impl ReplInput {
    pub fn new(program: &str, args: &[String]) -> Self {
        Self {
            program: program.to_string(),
            args: args.to_vec(),
        }
    }
}
