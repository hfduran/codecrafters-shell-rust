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

        let chars: Vec<char> = self.raw_string.chars().collect();

        let mut result: Vec<&str> = Vec::new();
        let mut foot: usize = 0;
        let mut head: usize = 0;
        let max = self.raw_string.len();

        while head < max {
            match chars[head] {
                SPACE => {
                    if foot == head {
                        head += 1;
                        foot += 1;
                    } else {
                        let word = &self.raw_string[foot..head];
                        result.push(word);
                        head += 1;
                        foot = head;
                    }
                }
                SINGLE_QUOTE => {
                    head += 1;
                    while head < max && chars[head] != SINGLE_QUOTE {
                        head += 1;
                    }
                    if head == max {
                        panic!("Didn't close the single quotes!");
                    }
                    let word = &self.raw_string[foot+1..head];
                    result.push(word);
                    head += 1;
                    foot = head;
                }
                _ => {
                    head += 1;
                }
            }
        }
        let word = &self.raw_string[foot..head];
        result.push(word);

        result.into_iter().map(String::from).collect()
    }
}
