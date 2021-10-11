#![allow(unused)]

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Link {
    link_text: String,
    href: String,
    title: String,
    kind: u8, // 0: normal link; 1: img
}

#[derive(Debug, Clone)]
pub enum Span {
    Code(String),
    Math(String),
    Emoji(String),
    Strong(String),
    Link(Link),
    Text(String),
}

pub fn parse_inline(input: &String) -> Vec<Span> {
    let mut spans = Vec::new();

    // Inline spans can not be nested.
    let mut open_tag = vec![];
    let mut text = String::from("");

    let mut indexs: Vec<usize> = Vec::new();
    let mut tags: Vec<char> = Vec::new();
    for (_index, _tag) in input.char_indices() {
        match _tag {
            '`' => match open_tag.last() {
                Some('`') => {
                    open_tag.pop();
                    indexs.push(_index);
                    tags.push(_tag);
                }
                None => {
                    open_tag.push('`');
                    indexs.push(_index);
                    tags.push(_tag);
                }
                _ => {}
            },
            '$' => match open_tag.last() {
                Some('$') => {
                    open_tag.pop();
                    indexs.push(_index);
                    tags.push(_tag);
                }
                None => {
                    open_tag.push('$');
                    indexs.push(_index);
                    tags.push(_tag);
                }
                _ => {}
            },
            ':' => match open_tag.last() {
                Some(':') => {
                    open_tag.pop();
                    indexs.push(_index);
                    tags.push(_tag);
                }
                None => {
                    open_tag.push(':');
                    indexs.push(_index);
                    tags.push(_tag);
                }
                _ => {}
            },
            '!' => match open_tag.last() {
                None => {
                    indexs.push(_index);
                    tags.push(_tag);
                }
                _ => {}
            },
            '*' => match open_tag.last() {
                Some('*') => {
                    open_tag.pop();
                    indexs.push(_index);
                    tags.push(_tag);
                }
                None => {
                    open_tag.push('*');
                    indexs.push(_index);
                    tags.push(_tag);
                }
                _ => {}
            },
            '[' => match open_tag.last() {
                None => {
                    open_tag.push(_tag);
                    indexs.push(_index);
                    tags.push(_tag);
                }
                _ => {}
            },
            ']' => match open_tag.last() {
                Some('[') => {
                    open_tag.pop();
                    indexs.push(_index);
                    tags.push(_tag);
                }
                _ => {}
            },
            '(' => match open_tag.last() {
                None => {
                    open_tag.push(_tag);
                    indexs.push(_index);
                    tags.push(_tag);
                }
                _ => {}
            },
            ')' => match open_tag.last() {
                Some('(') => {
                    open_tag.pop();
                    indexs.push(_index);
                    tags.push(_tag);
                }
                _ => {}
            },
            '<' => match open_tag.last() {
                None => {
                    open_tag.push(_tag);
                    indexs.push(_index);
                    tags.push(_tag);
                }
                _ => {}
            },
            '>' => match open_tag.last() {
                Some('<') => {
                    open_tag.pop();
                    indexs.push(_index);
                    tags.push(_tag);
                }
                _ => {}
            },
            '"' => match open_tag.last() {
                Some('(') => {
                    open_tag.push(_tag);
                    indexs.push(_index);
                    tags.push(_tag);
                }
                Some('"') => {
                    open_tag.pop();
                    indexs.push(_index);
                    tags.push(_tag);
                }
                _ => {}
            },
            _ => {}
        }
    }

    let mut idx = 0usize;
    let mut last = 0usize;
    let max_len = indexs.len();
    while idx < max_len {
        if tags[idx] == ':' && idx + 1 < max_len && tags[idx + 1] == ':' {
            // store the text
            if indexs[idx] - last > 0 {
                spans.push(Span::Text(String::from(&input[last..indexs[idx]])));
            }
            // store emoji
            spans.push(Span::Emoji(String::from(
                &input[indexs[idx] + 1..indexs[idx + 1]],
            )));
            // next
            last = indexs[idx + 1] + 1;
            idx += 2;
            continue;
        } else if tags[idx] == '$' && idx + 1 < max_len && tags[idx + 1] == '$' {
            // store the text
            if indexs[idx] - last > 0 {
                spans.push(Span::Text(String::from(&input[last..indexs[idx]])));
            }
            // store the text
            spans.push(Span::Math(String::from(
                &input[indexs[idx] + 1..indexs[idx + 1]],
            )));
            // next
            last = indexs[idx + 1] + 1;
            idx += 2;
            continue;
        } else if tags[idx] == '`' && idx + 1 < max_len && tags[idx + 1] == '`' {
            // store the text
            if indexs[idx] - last > 0 {
                spans.push(Span::Text(String::from(&input[last..indexs[idx]])));
            }
            // store code
            spans.push(Span::Code(String::from(
                &input[indexs[idx] + 1..indexs[idx + 1]],
            )));
            // next
            last = indexs[idx + 1] + 1;
            idx += 2;
            continue;
        } else if tags[idx] == '<' && idx + 1 < max_len && tags[idx + 1] == '>' {
            // store the text
            if indexs[idx] - last > 0 {
                spans.push(Span::Text(String::from(&input[last..indexs[idx]])));
            }
            // store link
            let text = String::from(&input[indexs[idx] + 1..indexs[idx + 1]]);
            spans.push(Span::Link(Link {
                link_text: text.to_owned(),
                href: text.to_owned(),
                title: text.to_owned(),
                kind: 0,
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
                spans.push(Span::Text(String::from(&input[last..indexs[idx]])));
            }
            // store Strong
            spans.push(Span::Strong(String::from(
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
                spans.push(Span::Text(String::from(&input[last..indexs[idx]])));
            }
            // store link
            spans.push(Span::Link(Link {
                link_text: String::from(&input[indexs[idx] + 1..indexs[idx + 1]]),
                href: String::from(&input[indexs[idx + 2] + 1..indexs[idx + 3]]),
                title: String::from(""),
                kind: 0,
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
                spans.push(Span::Text(String::from(&input[last..indexs[idx]])));
            }
            // store link
            spans.push(Span::Link(Link {
                link_text: String::from(&input[indexs[idx] + 1..indexs[idx + 1]]),
                href: String::from(&input[indexs[idx + 2] + 1..indexs[idx + 3]]),
                title: String::from(&input[indexs[idx + 3] + 1..indexs[idx + 4]]),
                kind: 0,
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
                spans.push(Span::Text(String::from(&input[last..indexs[idx]])));
            }
            // store img
            spans.push(Span::Link(Link {
                link_text: String::from(&input[indexs[idx + 1] + 1..indexs[idx + 2]]),
                href: String::from(&input[indexs[idx + 3] + 1..indexs[idx + 4]]),
                title: String::from(""),
                kind: 1,
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
                spans.push(Span::Text(String::from(&input[last..indexs[idx]])));
            }
            // store img
            spans.push(Span::Link(Link {
                link_text: String::from(&input[indexs[idx + 1] + 1..indexs[idx + 2]]),
                href: String::from(&input[indexs[idx + 3] + 1..indexs[idx + 4]]),
                title: String::from(&input[indexs[idx + 4] + 1..indexs[idx + 5]]),
                kind: 1,
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
        spans.push(Span::Text(the_rest));
    }

    // return result
    spans
}

#[cfg(test)]
mod tests {
    use super::parse_inline;
    use std::fs;

    #[test]
    fn test_inline() {
        // run with cargo test -- --nocapture
        let input = String::from("`hello`  :+1: hello **我是粗体**，`markx` is good，哈哈。$\\pi$，[链接](/hello/word \"hello\") ![图片](我是图片链接 \"标题\")，解析我");

        let spans = parse_inline(&input);
        println!("{:?}", spans);
    }

    #[test]
    fn test_inline2() {
        let input = String::from("`int(input(\"请输入你猜的数字：\"))`：`input()` 表示输出一段提示语句，然后以字符串形式接收你输入的内容，`int()` 表示把输入的内容转化为整数。");
        let spans = parse_inline(&input);
        println!("{:?}", spans);
    }
}
