#![allow(unused)]

use std::collections::HashMap;

#[derive(Debug)]
struct Mark(String);

#[derive(Debug)]
struct Block(String);

#[derive(Debug)]
pub struct BlockParser {
    marks: Vec<Mark>,
    blocks: Vec<Block>,
}

impl BlockParser {
    pub fn new() -> Self {
        BlockParser {
            marks: Vec::new(),
            blocks: Vec::new(),
        }
    }

    pub fn parse(&mut self, input: &String) {
        let mut open_tag = String::from("");
        let mut multi_line = String::from("");
        for line in input.lines() {
            if line.starts_with("#") {
                // Get the head block.
                println!("Get HEAD: {}", line);
            } else if line.starts_with("-") {
                // Get list item.
                println!("Get LIST: {}", line);
            } else if line.starts_with("|") {
                // Get table row
                println!("Get ROW: {}", line);
            } else if line.starts_with("```") {
                // Get fenced code block.
                // All lines will be gathered in the last `else` statement.
                // You can review the logic near `Paragraph Block`
                //
                // One thing need to note:
                // This can be a normal code block or
                // a block in `Pre Block, Exercise Block.`

                if open_tag.is_empty() {
                    // normal fenced code starts
                    open_tag += "```";
                    multi_line += line.trim_start_matches('`');
                    multi_line += "\n";
                } else if open_tag.starts_with("`") {
                    // normal fenced code ends
                    println!("Get CODE: {}", multi_line);
                    open_tag.clear();
                    multi_line.clear();
                } else {
                    // fenced code in some other block.
                    multi_line += line;
                    multi_line += "\n";
                }
            } else if line.starts_with("$$") {
                // The logic is same as fenced code
                if open_tag.is_empty() {
                    // display math starts.
                    open_tag += "$$";
                } else if open_tag.starts_with("$") {
                    // display math ends.
                    println!("Get Math: {}", multi_line);
                    open_tag.clear();
                    multi_line.clear();
                } else {
                    // display math in other block.
                    multi_line += line;
                    multi_line += "\n";
                }
            } else if line.starts_with("%%") {
                // `Pre Block` can not be contained in other block.
                if open_tag.is_empty() {
                    // pre block starts.
                    open_tag += "%%";
                    multi_line += line.trim_start_matches('%');
                    multi_line += "\n";
                } else {
                    // end of pre block.
                    println!("Get Pre: {}", multi_line);
                    open_tag.clear();
                    multi_line.clear();
                }
            } else {
                if open_tag.is_empty() {
                    // All normal line will be regarded as paragraph.
                    println!("Get P: {}", line);
                } else {
                    // These lines are in some multi-line blocks.
                    multi_line += line;
                    multi_line += "\n";
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
    use super::BlockParser;
    use std::fs;

    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }

    #[test]
    fn it_worked() {
        // run with cargo test -- --nocapture
        let input = fs::read_to_string("tests/test3.md").unwrap();
        let mut parser: BlockParser = BlockParser::new();
        parser.parse(&input);
        parser.show();
    }
}
