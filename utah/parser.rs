use crate::ParseToken;

pub struct Parser<'a> {
    pub rules: &'a Vec<Rule<'a>>
}

pub struct Rule<'a> {
    pub matches: Vec<&'a str>,
    pub tags: Vec<&'a str>,
    pub repeat: Option<usize>,
    pub add_all: bool
}

impl Parser<'_> {
    pub fn parse<'a>(&'a self, code: &mut Vec<ParseToken<'a>>, verbose: bool) {
        self.parse_depth(code, verbose, 0);
    }

    pub fn parse_depth<'a>(&'a self, code: &mut Vec<ParseToken<'a>>, verbose: bool, depth: usize) {
        if depth > 10_000 {
            panic!("Sanity check error: code parsed recursed too deeply");
        }

        let mut changed = false;

        for rule in self.rules {
            rule.traverse(code, &mut changed, verbose);
        }

        if changed { 
            if verbose {
                println!("Entering depth {}", depth + 1);
            }
            self.parse_depth(code, verbose, depth + 1); 
        }
    }
}

impl Rule<'_> {
    pub fn traverse<'a>(&'a self, code: &mut Vec<ParseToken<'a>>, changed: &mut bool, verbose: bool) {
        let mut start_index: usize = 0;

        'outer: while start_index < code.len() {
            let mut works = true;
            let mut index_in_rule = 0;
            let mut parse_token_index = 0;

            while index_in_rule < self.matches.len() {

                if parse_token_index + start_index >= code.len() {
                    if index_in_rule != self.matches.len() - 1 || Some(index_in_rule) != self.repeat {
                        works = false;
                    }
                    break;
                }
                if !code[parse_token_index + start_index].tags.contains(&self.matches[index_in_rule]) {
                    if Some(index_in_rule) == self.repeat {
                        if verbose {
                            println!("Matched the * index.");
                        }
                        index_in_rule += 1;
                        if index_in_rule >= self.matches.len() {
                            break;
                        }
                        continue;
                    }
                    else {
                        works = false;
                        break;
                    }
                }
                if Some(index_in_rule) != self.repeat {
                    index_in_rule += 1;
                }

                parse_token_index += 1;
            }

            if !works {
                start_index += 1;
                continue 'outer;
            }
            
            if self.add_all {
                self.add_all(code, start_index, start_index + parse_token_index, changed);
            }
            else {
                self.combine(code, start_index, start_index + parse_token_index, changed);
            }

            start_index += 1;
        }
    }

    pub fn combine<'a>(&'a self, code: &mut Vec<ParseToken<'a>>, start_index: usize, end_index: usize, changed: &mut bool) {
                code[start_index] = ParseToken {
                    location: code[start_index].location.start..code[end_index - 1].location.end,
                    body: code[0].body,
                    children: code[start_index..end_index].to_vec(),
                    tags: self.tags.clone(),
                    ..code[start_index]
                };
                for i in (start_index + 1)..end_index {
                    if i >= code.len() {
                        continue;
                    }

                    code.remove(i);
                    *changed = true;
                }
    }

    pub fn add_all<'a>(&'a self, code: &mut Vec<ParseToken<'a>>, start_index: usize, end_index: usize, changed: &mut bool) {
            for pt in start_index..end_index {
                if pt >= code.len() {
                    continue;
                }

                for t in &self.tags {
                    if !code[pt].tags.contains(t) {
                        code[pt].tags.push(t);
                        *changed = true;
                        println!("{}", code[pt]);
                    }
                }
            }
    }
}