use std::fmt::Display;

#[derive(Debug)]
pub struct Token<'a> {
    pub content: String,
    pub tags: Vec<&'a str>,
    pub line: usize,
    pub char: usize,
    pub file: &'a str
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{0}\" (line {2}, char {3} in {4}): {1:?}", self.content, self.tags, self.line, self.char, self.file)
    }
}

pub fn print_tokens(tokens: &[Token]) {
    for token in tokens.iter() {
        println!("{}", token);
    }
}
    
pub fn to_tokens<'a>(text: &'a str, file_name : &'a str) -> Vec<Token<'a>>
{
    let mut char_index: usize = 0;
    let mut line_index: usize = 0;
    let mut tokens : Vec<Token> = vec![];

    for i in 0..text.len() {
        let c = &text[i..=i];
        tokens.push(Token {
            content: c.to_owned(),
            tags : vec![c],
            line: line_index,
            char: char_index,
            file: file_name
        });

        if let Some(ch) = c.chars().nth(0) {
            let len = tokens.len();

            if ch.is_whitespace() {
                tokens[len - 1].tags.push("ws");
            } 
        }

        char_index += 1;
        if text.chars().nth(char_index) == Some('\n') {
            char_index = 0;
            line_index += 1;
        }
    }

    tokens.push(Token {
        content: String::from(""),
        tags: vec![" ", "ws"],
        line: line_index,
        char: char_index,
        file: file_name
    });

    tokens
}