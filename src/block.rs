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
        let mut is_open_tag = true;
        let mut muti_line = String::from("");
        for line in input.lines() {
            if line.starts_with("#") {
                println!("Get HEAD: {}", line);
            } else if line.starts_with(">") {
                println!("Get PRE: {}", line);
            } else if line.starts_with("-") {
                println!("Get LIST: {}", line);
            } else if line.starts_with("|") {
                println!("Get ROW: {}", line);
            } else if line.starts_with("```") {
                if is_open_tag {
                    is_open_tag = false;
                    muti_line += line;
                    muti_line += "\n";
                } else {
                    println!("Get CODE: {}", muti_line + line);
                    is_open_tag = true;
                    muti_line = String::from("");
                }
            } else if line.starts_with("$$") {
                println!("Get MATH: {}", line);
            } else {
                if is_open_tag {
                    println!("Get P: {}", line);
                } else {
                    muti_line += line;
                    muti_line += "\n";
                }
            }
        }
    }

    pub fn show(&self) {
        println!("{:?}", self.blocks);
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
        let input = fs::read_to_string("tests/test2.md").unwrap();
        let mut parser: Parser = Parser::new();
        parser.parse(&input);
        parser.show();
    }
}
