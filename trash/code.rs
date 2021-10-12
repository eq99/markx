#![allow(unused)]

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
    use super::parse_fenced_code;
    use std::fs;

    #[test]
    fn test_inline() {
        // run with cargo test -- --nocapture
        let input = String::from("python\r\nfor x in range(0, 20, 2):\nprint(x, end=\" \")");

        let code = parse_fenced_code(&input);
        println!("{:?}", code);
    }
}
