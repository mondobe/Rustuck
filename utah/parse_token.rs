use std::fmt::Display;

pub struct ParseToken<'a> {
    pub content: String,
    pub tags: Vec<&'a str>,
    pub children: Vec<&'a ParseToken<'a>>,
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