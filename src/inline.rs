#![allow(unused)]

use std::collections::HashMap;

#[derive(Debug)]
struct Link {
    inner_text: String,
    href: String,
    title: String,
}

#[derive(Debug)]
struct Img {
    alt: String,
    src: String,
    title: String,
}

#[derive(Debug)]
enum Span {
    Code(String),
    Math(String),
    Emoji(String),
    Strong(String),
    Link(Link),
    Img(Img),
    Text(String),
}

#[derive(Debug)]
pub struct InlineParser {
    spans: Vec<Span>,
}

#[derive(Debug)]
pub struct Point {
    index: usize,
    one_char: char,
}

impl InlineParser {
    pub fn new() -> Self {
        InlineParser { spans: Vec::new() }
    }

    pub fn parse(&mut self, input: &String) {
        // Inline spans can not be nested.

        let mut open_tag = String::from("");
        let mut text = String::from("");

        let mut indexs: Vec<usize> = Vec::new();
        let mut tags: Vec<char> = Vec::new();
        for (_index, _tag) in input.char_indices() {
            if "`$<>*:![]()\"".contains(_tag) {
                indexs.push(_index);
                tags.push(_tag);
            }
        }

        let mut idx = 0usize;
        let mut last = 0usize;
        let max_len = indexs.len();
        while idx < max_len {
            if tags[idx] == ':' && idx + 1 < max_len && tags[idx + 1] == ':' {
                // store the text
                if indexs[idx] - last > 0 {
                    self.spans
                        .push(Span::Text(String::from(&input[last..indexs[idx]])));
                }
                // store emoji
                self.spans.push(Span::Emoji(String::from(
                    &input[indexs[idx] + 1..indexs[idx + 1]],
                )));
                // next
                last = indexs[idx + 1] + 1;
                idx += 2;
                continue;
            } else if tags[idx] == '$' && idx + 1 < max_len && tags[idx + 1] == '$' {
                // store the text
                if indexs[idx] - last > 0 {
                    self.spans
                        .push(Span::Text(String::from(&input[last..indexs[idx]])));
                }
                // store the text
                self.spans.push(Span::Math(String::from(
                    &input[indexs[idx] + 1..indexs[idx + 1]],
                )));
                // next
                last = indexs[idx + 1] + 1;
                idx += 2;
                continue;
            } else if tags[idx] == '`' && idx + 1 < max_len && tags[idx + 1] == '`' {
                // store the text
                if indexs[idx] - last > 0 {
                    self.spans
                        .push(Span::Text(String::from(&input[last..indexs[idx]])));
                }
                // store code
                self.spans.push(Span::Code(String::from(
                    &input[indexs[idx] + 1..indexs[idx + 1]],
                )));
                // next
                last = indexs[idx + 1] + 1;
                idx += 2;
                continue;
            } else if tags[idx] == '<' && idx + 1 < max_len && tags[idx + 1] == '>' {
                // store the text
                if indexs[idx] - last > 0 {
                    self.spans
                        .push(Span::Text(String::from(&input[last..indexs[idx]])));
                }
                // store link
                let text = String::from(&input[indexs[idx] + 1..indexs[idx + 1]]);
                self.spans.push(Span::Link(Link {
                    inner_text: text.to_owned(),
                    href: text.to_owned(),
                    title: text.to_owned(),
                }));
                // next
                last = indexs[idx + 1] + 1;
                idx += 2;
                continue;
            } else if tags[idx] == '*'
                && idx + 3 < max_len
                && tags[idx + 1] == '*'
                && tags[idx + 2] == '*'
                && tags[idx + 3] == '*'
                && indexs[idx + 1] == indexs[idx] + 1
                && indexs[idx + 3] == indexs[idx + 2] + 1
            {
                // store the text
                if indexs[idx] - last > 0 {
                    self.spans
                        .push(Span::Text(String::from(&input[last..indexs[idx]])));
                }
                // store Strong
                self.spans.push(Span::Strong(String::from(
                    &input[indexs[idx + 1] + 1..indexs[idx + 2]],
                )));
                // next
                last = indexs[idx + 3] + 1;
                idx += 4;
                continue;
            } else if tags[idx] == '['
                && idx + 3 < max_len
                && tags[idx + 1] == ']'
                && tags[idx + 2] == '('
                && tags[idx + 3] == ')'
                && indexs[idx + 2] == indexs[idx + 1] + 1
            {
                // store the text
                if indexs[idx] - last > 0 {
                    self.spans
                        .push(Span::Text(String::from(&input[last..indexs[idx]])));
                }
                // store link
                self.spans.push(Span::Link(Link {
                    inner_text: String::from(&input[indexs[idx] + 1..indexs[idx + 1]]),
                    href: String::from(&input[indexs[idx + 2] + 1..indexs[idx + 3]]),
                    title: String::from(""),
                }));
                // next
                last = indexs[idx + 3] + 1;
                idx += 4;
                continue;
            } else if tags[idx] == '['
                && idx + 3 < max_len
                && tags[idx + 1] == ']'
                && tags[idx + 2] == '('
                && tags[idx + 3] == '\"'
                && tags[idx + 4] == '\"'
                && tags[idx + 5] == ')'
                && indexs[idx + 2] == indexs[idx + 1] + 1
            {
                // store the text
                if indexs[idx] - last > 0 {
                    self.spans
                        .push(Span::Text(String::from(&input[last..indexs[idx]])));
                }
                // store link
                self.spans.push(Span::Link(Link {
                    inner_text: String::from(&input[indexs[idx] + 1..indexs[idx + 1]]),
                    href: String::from(&input[indexs[idx + 2] + 1..indexs[idx + 3]]),
                    title: String::from(&input[indexs[idx + 3] + 1..indexs[idx + 4]]),
                }));
                // next
                last = indexs[idx + 5] + 1;
                idx += 6;
                continue;
            } else if tags[idx] == '!'
                && idx + 4 < max_len
                && tags[idx + 1] == '['
                && tags[idx + 2] == ']'
                && tags[idx + 3] == '('
                && tags[idx + 4] == ')'
                && indexs[idx + 1] == indexs[idx] + 1
                && indexs[idx + 3] == indexs[idx + 2] + 1
            {
                // store the text
                if indexs[idx] - last > 0 {
                    self.spans
                        .push(Span::Text(String::from(&input[last..indexs[idx]])));
                }
                // store img
                self.spans.push(Span::Img(Img {
                    alt: String::from(&input[indexs[idx + 1] + 1..indexs[idx + 2]]),
                    src: String::from(&input[indexs[idx + 3] + 1..indexs[idx + 4]]),
                    title: String::from(""),
                }));
                // next
                last = indexs[idx + 4] + 1;
                idx += 5;
                continue;
            } else if tags[idx] == '!'
                && idx + 4 < max_len
                && tags[idx + 1] == '['
                && tags[idx + 2] == ']'
                && tags[idx + 3] == '('
                && tags[idx + 4] == '\"'
                && tags[idx + 5] == '\"'
                && tags[idx + 6] == ')'
                && indexs[idx + 1] == indexs[idx] + 1
                && indexs[idx + 3] == indexs[idx + 2] + 1
            {
                // store the text
                if indexs[idx] - last > 0 {
                    self.spans
                        .push(Span::Text(String::from(&input[last..indexs[idx]])));
                }
                // store img
                self.spans.push(Span::Img(Img {
                    alt: String::from(&input[indexs[idx + 1] + 1..indexs[idx + 2]]),
                    src: String::from(&input[indexs[idx + 3] + 1..indexs[idx + 4]]),
                    title: String::from(&input[indexs[idx + 4] + 1..indexs[idx + 5]]),
                }));
                // next
                last = indexs[idx + 6] + 1;
                idx += 7;
                continue;
            } else {
                // skip
                idx += 1;
            }
        }

        let the_rest = String::from(&input[last..]);
        if the_rest.len() > 0 {
            self.spans.push(Span::Text(the_rest));
        }
    }

    pub fn show(&self) {
        println!("{:?}", self.spans);
    }
}
#[cfg(test)]
mod tests {
    use super::InlineParser;
    use std::fs;

    #[test]
    fn test_inline() {
        // run with cargo test -- --nocapture
        let input = String::from(":+1: hello **我是粗体**，`markx` is good，哈哈。$\\pi$，[链接](/hello/word \"hello\") ![图片](我是图片链接 \"标题\")，解析我");
        let mut parser: InlineParser = InlineParser::new();
        parser.parse(&input);
        parser.show();
    }
}
