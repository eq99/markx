#![allow(unused)]

use crate::block::*;
use crate::inline::{parse_inline, Span};

pub fn mark2html(input: &String) -> String {
    let mut html = String::from("");
    let mut ul = String::from("");
    let blocks = parse_doc(input);

    if blocks.is_empty() {
        html = format!("");
    } else {
        for block in blocks {
            match block {
                Block::LeafBlock(leaf) => match leaf {
                    LeafBlock::ListItem(list) => {
                        ul = format!("{}{}", ul, list.tohtml());
                    }
                    LeafBlock::Paragraph(leaf) => {
                        if ul.is_empty() {
                            html = format!("{}{}", html, leaf.tohtml());
                        } else {
                            html = format!("{}<ul>{}</ul>{}", html, ul, leaf.tohtml());
                            ul.clear();
                        }
                    }
                    LeafBlock::DisplayMath(leaf) => {
                        if ul.is_empty() {
                            html = format!("{}{}", html, leaf.tohtml());
                        } else {
                            html = format!("{}<ul>{}</ul>{}", html, ul, leaf.tohtml());
                            ul.clear();
                        }
                    }
                    LeafBlock::FencedCode(leaf) => {
                        if ul.is_empty() {
                            html = format!("{}{}", html, leaf.tohtml());
                        } else {
                            html = format!("{}<ul>{}</ul>{}", html, ul, leaf.tohtml());
                            ul.clear();
                        }
                    }
                    LeafBlock::Heading(leaf) => {
                        if ul.is_empty() {
                            html = format!("{}{}", html, leaf.tohtml());
                        } else {
                            html = format!("{}<ul>{}</ul>{}", html, ul, leaf.tohtml());
                            ul.clear();
                        }
                    }
                },
                Block::ContainerBlock(c) => {
                    html = format!("{}{}", html, c.tohtml());
                }
            }
        }

        // gather last ul
        if !ul.is_empty() {
            html = format!("{}<ul>{}</ul>", html, ul);
        }
    }

    html
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_html() {
        // run with cargo test -- --nocapture
        let input = fs::read_to_string("tests/test4.md").unwrap();

        let html = mark2html(&input);
        println!("{:?}", html);
    }
}
