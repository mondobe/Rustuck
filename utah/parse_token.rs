use std::fmt::Display;
use crate::Token;

#[derive(Clone)]
pub struct ParseToken<'a> {
    pub content: String,
    pub tags: Vec<&'a str>,
    pub children: Vec<ParseToken<'a>>,
    pub line: usize,
    pub char: usize,
    pub file: &'a str
}

impl Display for ParseToken<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\"{0}\" (line {2}, char {3} in {4}): {1:?} {{", self.content, self.tags, self.line, self.char, self.file).ok();
        for pt in &self.children {
            writeln!(f, "{}", pt).ok();
        }
        writeln!(f, "}}").ok();
        Ok(())
    }
}

impl <'a>From<Token<'a>> for ParseToken<'a> {
    fn from(value: Token<'a>) -> Self {
        ParseToken { 
            content: value.content, 
            tags: value.tags, 
            children: vec![], 
            line: value.line, 
            char: value.char, 
            file: value.file 
        }
    }
}

pub fn to_parse_tokens(tokens: Vec<Token>) -> Vec<ParseToken> {
    let mut to_ret: Vec<ParseToken> = vec![];
    for token in tokens {
        to_ret.push(ParseToken::from(token));
    }
    to_ret
}

pub fn print_parse_tokens(pts: Vec<ParseToken>) {
    for pt in pts.iter() {
        println!("{}", pt);
    }
}