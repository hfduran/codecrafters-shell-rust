const SPACE: char = ' ';
const SINGLE_QUOTE: char = '\'';
const DOUBLE_QUOTE: char = '\"';
const ESCAPE_BAR: char = '\\';
const FORWARD: char = '>';
const PIPE: char = '|';

pub enum Operator {
    Forward,
    Pipe,
}

pub enum InputPart {
    Word(String),
    Operator(Operator),
}

pub struct ParsedInput {
    parts: Vec<InputPart>,
}

impl ParsedInput {
    fn new() -> Self {
        Self { parts: Vec::new() }
    }

    fn add_word(&mut self, word: &str) {
        self.parts.push(InputPart::Word(word.to_string()));
    }

    fn add_operator(&mut self, operator: char) {
        match operator {
            FORWARD => {
                self.parts.push(InputPart::Operator(Operator::Forward));
            }
            PIPE => {
                self.parts.push(InputPart::Operator(Operator::Pipe));
            }
            _ => {}
        }
    }

    fn words_values(&mut self) -> Vec<String> {
        self.parts.iter().filter_map(|x| match x {
            InputPart::Operator(_) => {None},
            InputPart::Word(val) => {Some(val.clone())}
        }).collect()
    }
}

pub fn extract_input_parts(string: &str) -> Vec<String> {
    if string == "" {
        return Vec::new();
    }

    let mut is_previous_a_space: bool = false;
    let mut is_previous_a_scape: bool = false;
    let mut is_single_quote_open: bool = false;
    let mut is_double_quote_open: bool = false;

    let mut word: String = String::new();
    let mut real_result: ParsedInput = ParsedInput::new();

    for c in string.chars() {
        match c {
            SPACE => {
                if is_previous_a_scape {
                    word.push(c);
                    is_previous_a_scape = false;
                    continue;
                }
                if is_single_quote_open || is_double_quote_open {
                    word.push(c);
                    continue;
                }
                if !is_previous_a_space {
                    real_result.add_word(&word);
                    word = String::new();
                }
                is_previous_a_space = true;
            }
            SINGLE_QUOTE => {
                if is_previous_a_scape && is_double_quote_open {
                    word.push(ESCAPE_BAR);
                    word.push(c);
                    is_previous_a_scape = false;
                    continue;
                }
                if is_previous_a_scape {
                    word.push(c);
                    is_previous_a_scape = false;
                    continue;
                }
                if is_double_quote_open {
                    word.push(c);
                    continue;
                }
                is_single_quote_open = !is_single_quote_open;
            }
            DOUBLE_QUOTE => {
                if is_previous_a_scape {
                    word.push(c);
                    is_previous_a_scape = false;
                    continue;
                }
                if is_single_quote_open {
                    word.push(c);
                    continue;
                }
                is_double_quote_open = !is_double_quote_open;
            }
            ESCAPE_BAR => {
                if is_single_quote_open {
                    word.push(c);
                    continue;
                }
                if is_previous_a_scape {
                    word.push(c);
                    is_previous_a_scape = false;
                } else {
                    is_previous_a_scape = true;
                }
            }
            FORWARD => {
                if !is_single_quote_open && !is_double_quote_open {
                    real_result.add_word(&word);
                    word = String::new();
                    real_result.add_operator(c);
                }
            }
            _ => {
                word.push(c);
                is_previous_a_space = false;
                is_previous_a_scape = false;
            }
        }
    }
    real_result.add_word(&word);
    real_result.words_values()
}
