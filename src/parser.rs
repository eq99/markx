#![allow(unused)]

use std::collections::HashMap;

#[derive(Debug)]
struct Mark(String);

#[derive(Debug)]
struct Block(String);

#[derive(Debug)]
pub struct Parser {
    marks: Vec<Mark>,
    blocks: Vec<Block>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            marks: Vec::new(),
            blocks: Vec::new(),
        }
    }

    pub fn parse(&mut self, input: &String) {
        let mut mark = Vec::new();
        let mut start = 0usize;
        let mut pre = 0usize;
        let mut inline = false;
        for (idx, char) in input.char_indices() {
            match char {
                '#' => {
                    if !inline || idx - pre == 1 {
                        println!("Head start...");
                        mark.push(char);
                        pre = idx;
                    }
                }
                '`' => {
                    if idx - pre == 1 {
                        // may start with
                        println!("Get `");
                        mark.push(char);
                    }
                    pre = idx;
                }
                '$' => {
                    if idx - pre == 1 {
                        // may start with
                        println!("Get $");
                        mark.push(char);
                    }
                    pre = idx;
                }
                '>' => {
                    if !inline {
                        println!("pre start...");
                        mark.push(char);
                    }
                }
                '-' => {
                    if !inline {
                        println!("List start...");
                        mark.push(char);
                    }
                }
                '|' => {
                    println!("table")
                }
                '\n' => {
                    if !mark.is_empty() {
                        match mark[0] {
                            '#' => {
                                /* header
                                 * start with 1-6 '#',
                                 * end with 'newline'('\r\n')
                                 */
                                let end = idx + char.len_utf8();
                                println!("Get head {:?}", &input[start..end]);
                                self.blocks.push(Block(String::from(&input[start..end])));
                                start = end;
                                mark.clear();
                                inline = false;
                            }
                            '-' => {
                                /** list
                                 * start with '-'
                                 * end with 'newline'('\r\n')
                                 */
                                let end = idx + char.len_utf8();
                                println!("Get list {:?}", &input[start..end]);
                                self.blocks.push(Block(String::from(&input[start..end])));
                                start = end;
                                mark.clear();
                                inline = false;
                            }
                            '>' => {
                                /** Pre
                                 * start with '>'
                                 * end with 'newline'('\r\n')
                                 */
                                let end = idx + char.len_utf8();
                                println!("Get Pre {:?}", &input[start..end]);
                                self.blocks.push(Block(String::from(&input[start..end])));
                                start = end;
                                mark.clear();
                                inline = false;
                            }
                            '`' => {
                                /** fenced code
                                 * start with '```'
                                 * end with '```newline'('\r\n')
                                 */
                                if mark.len() >= 4 {
                                    let end = idx + char.len_utf8();
                                    println!("Get Fenced Code {:?}", &input[start..end]);
                                    self.blocks.push(Block(String::from(&input[start..end])));
                                    start = end;
                                    mark.clear();
                                    inline = false;
                                }
                            }
                            '$' => {
                                /** Display math
                                 * start with '$$'
                                 * end with '$$newline'('\r\n')
                                 */
                                if mark.len() >= 2 {
                                    let end = idx + char.len_utf8();
                                    println!("Get display math {:?}", &input[start..end]);
                                    self.blocks.push(Block(String::from(&input[start..end])));
                                    start = end;
                                    mark.clear();
                                    inline = false;
                                }
                            }
                            _ => (),
                        }
                    } else {
                        /* paragraph
                         * start with '',
                         * end with 'newline'('\r\n')
                         */
                        let end = idx + char.len_utf8();
                        println!("Get paragraph {:?}", &input[start..end]);
                        self.blocks.push(Block(String::from(&input[start..end])));
                        start = end;
                        mark.clear();
                        inline = false;
                    }
                }
                _ => {
                    inline = true;
                }
            }
        }
    }

    pub fn show(&self) {
        println!("{:?}", self.marks);
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use std::fs;

    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }

    #[test]
    fn it_worked() {
        // run with cargo test -- --nocapture
        let input = fs::read_to_string("tests/test3.md").unwrap();
        let mut parser: Parser = Parser::new();
        parser.parse(&input);
        parser.show();
    }
}
