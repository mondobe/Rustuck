pub struct Parser<'a> {
    pub rules: &'a Vec<Rule<'a>>
}

pub struct Rule<'a> {
    pub matches: Vec<&'a str>,
    pub repeat: Option<usize>,
    pub add_all: bool
}