#![allow(unused)]

use crate::inline::{parse_inline, Span};

#[derive(Debug)]
struct Block(String);

#[derive(Debug)]
pub struct BlockParser {
    blocks: Vec<Block>,
}

pub fn parse_doc(input: &String) {
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
                // start of normal fenced code.
                open_tag += "```";
                multi_line += line.trim_start_matches('`');
                multi_line += "\r\n";
            } else if open_tag.starts_with("`") {
                // end of normal fenced code.
                println!("Get CODE: {}", multi_line);
                open_tag.clear();
                multi_line.clear();
            } else {
                // fenced code is in some other blocks.
                multi_line += line;
                multi_line += "\n";
            }
        } else if line.starts_with("$$") {
            // The logic is same as fenced code.
            if open_tag.is_empty() {
                // start of display math.
                open_tag += "$$";
            } else if open_tag.starts_with("$") {
                // end of display math.
                println!("Get Math: {}", multi_line);
                open_tag.clear();
                multi_line.clear();
            } else {
                // display math is in other block.
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

#[derive(Debug)]
pub struct Heading {
    level: usize,
    spans: Vec<Span>,
}

pub fn parse_heading(input: &String) -> Heading {
    let head_marks = input.split_whitespace().next().unwrap_or("");
    let head_body = input
        .strip_prefix(&format!("{} ", head_marks))
        .unwrap_or("");

    let spans = parse_inline(&String::from(head_body));

    Heading {
        level: head_marks.len(),
        spans: spans,
    }
}

#[derive(Debug)]
pub struct FencedCode {
    lang: String,
    code: String,
}

/// Note: the first line of the input contains meta data about the code.
/// and the first line is seprated by '\r\n'
pub fn parse_fenced_code(input: &String) -> FencedCode {
    let mut splitor = input.split("\r\n");
    let meta_line = splitor.next().unwrap_or("");
    let code = splitor.next().unwrap_or("");

    let lang = meta_line.split_whitespace().next().unwrap_or("");
    FencedCode {
        lang: String::from(lang),
        code: String::from(code),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }

    #[test]
    fn test_blocks() {
        // run with cargo test -- --nocapture
        let input = fs::read_to_string("tests/test2.md").unwrap();
        //println!("{:?}", input);
        parse_doc(&input);
    }

    fn test_heading() {
        // run with cargo test -- --nocapture
        let input = String::from("# hello $math$");

        let head = parse_heading(&input);
        println!("{:?}", head);
    }

    #[test]
    fn test_fenced_code() {
        // run with cargo test -- --nocapture
        let input = String::from("python\r\nfor x in range(0, 20, 2):\nprint(x, end=\" \")");

        let code = parse_fenced_code(&input);
        println!("{:?}", code);
    }
}
