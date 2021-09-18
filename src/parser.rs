#![allow(unused)]

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
        self.marks.push(Mark(String::from("**")));
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
        let input = fs::read_to_string("tests/test.md").unwrap();
        println!("{:?}", input);
        print_type_of(&input);

        let mut parser: Parser = Parser::new();
        parser.parse(&input);
        parser.show();
    }
}
