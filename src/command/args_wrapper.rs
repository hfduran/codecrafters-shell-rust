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

    #[rustfmt::skip]
    pub fn get_args_vec(&self) -> Vec<String> {
        const SPACE: char = ' ';
        const SINGLE_QUOTE: char = '\'';
        const DOUBLE_QUOTE: char = '\"';

        let chars: Vec<char> = self.raw_string.chars().collect();

        let mut result: Vec<String> = Vec::new();
        let mut foot: usize = 0;
        let mut head: usize = 0;
        let max = self.raw_string.len();

        while head < max {
            match chars[head] {
                SPACE => {
                    if foot == head {
                        /* arg1  arg2
                                ^ head
                                ^ foot  */
                        head += 1;
                        foot += 1;
                    } else {
                        /* arg1  arg2
                           ^   ^ head
                           L foot       */
                        let word = &self.raw_string[foot..head];
                        result.push(word.to_string());
                        head += 1;
                        foot = head;
                    }
                }
                SINGLE_QUOTE => {
                    /* 'hello there' hi
                       ^ head
                       ^ foot              */
                    head += 1;
                    while head < max && chars[head] != SINGLE_QUOTE {
                        head += 1;
                    }

                    if head == max {
                        /* 'hello there
                           ^           ^ head
                           L foot              */
                        panic!("Didn't close the single quotes!");
                    }

                    /* 'hello there' hi
                       ^           ^ head
                       L foot              */

                    while head < max && chars[head] != SPACE {
                        /* 'hello there'include exclude
                           ^           ^~~~~~~~ > this should be included!
                           |           L head
                           L foot               
                        result: "hello thereinclude", "exclude"     */
                        head += 1;
                    }

                    /* 'hello there' hi
                       ^            ^ head
                       L foot              */

                    let word = self.raw_string[foot..head]
                        .chars()
                        .filter(|c| *c != SINGLE_QUOTE)
                        .collect::<String>();
                    result.push(word);

                    head += 1;
                    foot = head;

                    /* 'hello there' hi
                                     ^ head
                                     L foot      */

                }
                DOUBLE_QUOTE => {
                    /* 'hello there' hi
                       ^ head
                       ^ foot              */
                    head += 1;
                    while head < max && chars[head] != DOUBLE_QUOTE {
                        head += 1;
                    }

                    if head == max {
                        /* 'hello there
                           ^           ^ head
                           L foot              */
                        panic!("Didn't close the single quotes!");
                    }

                    /* 'hello there' hi
                       ^           ^ head
                       L foot              */

                    while head < max && chars[head] != SPACE {
                        /* 'hello there'include exclude
                           ^           ^~~~~~~~ > this should be included!
                           |           L head
                           L foot               
                        result: "hello thereinclude", "exclude"     */
                        head += 1;
                    }

                    /* 'hello there' hi
                       ^            ^ head
                       L foot              */

                    let word = self.raw_string[foot..head]
                        .chars()
                        .filter(|c| *c != DOUBLE_QUOTE)
                        .collect::<String>();
                    result.push(word);

                    head += 1;
                    foot = head;

                    /* 'hello there' hi
                                     ^ head
                                     L foot      */

                }
                _ => {
                    head += 1;
                }
            }
       }

        if foot != head {
            let word = &self.raw_string[foot..head];
            result.push(word.to_string());
        }

        result
    }
}
