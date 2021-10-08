#![allow(unused)]

use crate::inline::parse_inline;
use crate::inline::Span;

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

#[cfg(test)]
mod tests {
    use super::parse_heading;
    use std::fs;

    #[test]
    fn test_inline() {
        // run with cargo test -- --nocapture
        let input = String::from("# hello $math$");

        let head = parse_heading(&input);
        println!("{:?}", head);
    }
}
