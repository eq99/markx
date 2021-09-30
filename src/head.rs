#![allow(unused)]

#[derive(Debug)]
struct Mark(String);

#[derive(Debug)]
struct Head(String);

#[derive(Debug)]
pub struct HeadParser {
    marks: Vec<Mark>,
    heads: Vec<Head>,
}

impl HeadParser {
    pub fn new() -> Self {
        HeadParser {
            marks: Vec::new(),
            heads: Vec::new(),
        }
    }
}
