#![allow(unused)]

use crate::inline::{parse_inline, Span};

#[derive(Debug, Clone)]
pub struct Heading {
    level: usize,
    spans: Vec<Span>,
}

pub fn parse_heading(input: &String) -> Heading {
    let raw_head = input.trim_start_matches('#');

    Heading {
        level: input.len() - raw_head.len(),
        spans: parse_inline(&String::from(raw_head.trim_end_matches('#').trim())),
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct DisplayMath(String);

pub fn parse_display_math(input: &String) -> DisplayMath {
    DisplayMath(String::from(input))
}

#[derive(Debug, Clone)]
pub struct ListItem(Vec<Span>);

pub fn parse_list_item(input: &String) -> ListItem {
    ListItem(parse_inline(&String::from(
        input.strip_prefix("- ").unwrap_or(""),
    )))
}

#[derive(Debug, Clone)]
pub struct Paragraph(Vec<Span>);

pub fn parse_paragraph(input: &String) -> Paragraph {
    Paragraph(parse_inline(input))
}

#[derive(Debug, Clone)]
pub struct Pre {
    name: String,
    title: String,
    leafs: Vec<LeafBlock>,
}

pub fn parse_pre(input: &String) -> Pre {
    let mut splitor = input.split("\r\n");
    let meta_line = splitor.next().unwrap_or("");
    let content = splitor.next().unwrap_or("");

    let mut meta_splitor = meta_line.split_whitespace();

    let name = meta_splitor.next().unwrap_or("");
    let title = meta_splitor.next().unwrap_or("");

    Pre {
        name: String::from(name),
        title: String::from(title),
        leafs: vec![],
    }
}

#[derive(Debug, Clone)]
pub enum LeafBlock {
    Heading(Heading),
    FencedCode(FencedCode),
    DisplayMath(DisplayMath),
    Paragraph(Paragraph),
    ListItem(ListItem),
}

#[derive(Debug)]
pub enum ContainerBlock {
    Pre(Pre),
}

#[derive(Debug)]
pub enum Block {
    LeafBlock(LeafBlock),
    ContainerBlock(ContainerBlock),
}

pub fn parse_doc(input: &String) -> Vec<Block> {
    let mut blocks = vec![];
    let mut pre = Pre {
        name: String::from(""),
        title: String::from(""),
        leafs: vec![],
    };

    let mut multi_line = String::from("");
    let mut tag_stack = vec![];
    let mut container = ' ';

    for line in input.lines() {
        if line.starts_with("# ") || line.starts_with("## ") || line.starts_with("### ") {
            // Get heading.
            // Note: Only h1, h2, h3 is allowed.
            let h = parse_heading(&String::from(line));
            blocks.push(Block::LeafBlock(LeafBlock::Heading(h)));
        } else if line.starts_with("- ") {
            // Get list item.
            let list_item = parse_list_item(&String::from(line));
            blocks.push(Block::LeafBlock(LeafBlock::ListItem(list_item)));
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
            match tag_stack.last() {
                Some('`') => {
                    // Get fenced code.
                    let fenced_code = parse_fenced_code(&String::from(&multi_line));
                    match container {
                        ' ' => {
                            blocks.push(Block::LeafBlock(LeafBlock::FencedCode(fenced_code)));
                        }
                        '%' => {
                            pre.leafs.push(LeafBlock::FencedCode(fenced_code));
                        }
                        _ => {}
                    }
                    tag_stack.pop();
                    multi_line.clear();
                }
                _ => {
                    // start of fenced code
                    tag_stack.push('`');
                    multi_line = format!("{} {}\r\n", multi_line, line.trim_start_matches('`'));
                }
            }
        } else if line.starts_with("$$") {
            match tag_stack.last() {
                Some('$') => {
                    // Get display math.
                    let display_math = parse_display_math(&String::from(&multi_line));
                    match container {
                        ' ' => {
                            blocks.push(Block::LeafBlock(LeafBlock::DisplayMath(display_math)));
                        }
                        '%' => {
                            pre.leafs.push(LeafBlock::DisplayMath(display_math));
                        }
                        _ => {}
                    }
                    tag_stack.pop();
                    multi_line.clear();
                }
                _ => {
                    // start of display math
                    tag_stack.push('$');
                }
            }
        } else if line.starts_with("%%") {
            // Get pre block
            // Note: Pre block is a container block.
            match tag_stack.last() {
                None => {
                    // start of pre
                    tag_stack.push('%');
                    container = '%';
                    let name = line.trim_start_matches('%').split(" ").next().unwrap_or("");
                    pre.title = String::from(
                        line.trim_start_matches("%")
                            .strip_prefix(name)
                            .unwrap_or("")
                            .trim(),
                    );
                    pre.name = String::from(name);
                }
                Some('%') => {
                    // end of pre
                    blocks.push(Block::ContainerBlock(ContainerBlock::Pre(pre.clone())));
                    pre.leafs.clear();
                    tag_stack.pop();
                    container = ' ';
                }
                _ => {}
            }
        } else {
            match tag_stack.last() {
                Some('%') => {
                    // this line is in pre block.
                    pre.leafs
                        .push(LeafBlock::Paragraph(parse_paragraph(&String::from(line))));
                }
                None => {
                    // Get normal line, which will be take as paragraph
                    blocks.push(Block::LeafBlock(LeafBlock::Paragraph(parse_paragraph(
                        &String::from(line),
                    ))));
                }
                _ => {
                    // this line is in some block.
                    multi_line = format!("{}{}\n", multi_line, line);
                }
            }
        }
    }

    blocks
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
        let input = fs::read_to_string("tests/test4.md").unwrap();
        //println!("{:?}", input);
        println!("{:?}", parse_doc(&input));
    }

    #[test]
    #[ignore]
    fn test_heading() {
        // run with cargo test -- --nocapture
        let input = String::from("# hello $math$");

        let head = parse_heading(&input);
        println!("{:?}", head);
    }

    #[test]
    #[ignore]
    fn test_fenced_code() {
        // run with cargo test -- --nocapture
        let input = String::from("python\r\nfor x in range(0, 20, 2):\nprint(x, end=\" \")");

        let code = parse_fenced_code(&input);
        println!("{:?}", code);
    }
}
