pub fn extract_string_parts(string: &str) -> Vec<String> {
    if string == "" {
        return Vec::new();
    }

    const SPACE: char = ' ';
    const SINGLE_QUOTE: char = '\'';
    const DOUBLE_QUOTE: char = '\"';
    const ESCAPE_BAR: char = '\\';

    let mut is_previous_a_space: bool = false;
    let mut is_previous_a_scape: bool = false;
    let mut is_single_quote_open: bool = false;
    let mut is_double_quote_open: bool = false;

    let mut word: String = String::new();
    let mut result: Vec<String> = Vec::new();

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
                    result.push(word.clone());
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
            _ => {
                word.push(c);
                is_previous_a_space = false;
                is_previous_a_scape = false;
            }
        }
    }
    result.push(word.to_string());
    result
}
