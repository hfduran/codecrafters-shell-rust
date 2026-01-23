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
        const SPACE: char = ' ';
        const SINGLE_QUOTE: char = '\'';
        const DOUBLE_QUOTE: char = '\"';

        let mut is_previous_a_space: bool = false;
        let mut is_single_quote_open: bool = false;
        let mut is_double_quote_open: bool = false;

        let mut word: String = String::new();
        let mut result: Vec<String> = Vec::new();

        for c in self.raw_string.chars() {
            match c {
                SPACE => {
                    if is_single_quote_open || is_double_quote_open {
                        word.push(c);
                        continue;
                    }
                    if !is_previous_a_space {
                        result.push(word.clone());
                        word = String::new();
                    }
                    is_previous_a_space = true;
                }
                SINGLE_QUOTE => {
                    if is_double_quote_open {
                        word.push(c);
                        continue;
                    }
                    is_single_quote_open = !is_single_quote_open;
                }
                DOUBLE_QUOTE => {
                    if is_single_quote_open {
                        word.push(c);
                        continue;
                    }
                    is_double_quote_open = !is_double_quote_open;
                }
                _ => {
                    is_previous_a_space = false;
                    word.push(c);
                }
            }
        }
        result.push(word.to_string());
        result
    }
}
