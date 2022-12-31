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
            if rule.add_all {
                rule.traverse_and_add_all(code, &mut changed, verbose)
            }
            else {
                rule.traverse_and_combine(code, &mut changed, verbose);
            }
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
    pub fn traverse_and_combine<'a>(&'a self, code: &mut Vec<ParseToken<'a>>, changed: &mut bool, verbose: bool) {
        let mut start_index: usize = 0;

        *changed = false;

        'outer: while start_index < code.len() {
            let mut buffer: Vec<ParseToken> = vec![];
            let mut works = true;
            let mut r_index = 0;
            let mut p_index = 0;

            while r_index < self.matches.len() {

                if p_index + start_index >= code.len() {
                    if r_index != self.matches.len() - 1 || Some(r_index) != self.repeat {
                        works = false;
                    }
                    break;
                }
                if !code[p_index + start_index].tags.contains(&self.matches[r_index]) {
                    if Some(r_index) == self.repeat {
                        if verbose {
                            println!("Matched the * index.");
                        }
                        r_index += 1;
                        if r_index >= self.matches.len() {
                            break;
                        }
                        continue;
                    }
                    else {
                        works = false;
                        break;
                    }
                }
                buffer.push(code[p_index + start_index].clone());
                if Some(r_index) != self.repeat {
                    r_index += 1;
                }

                p_index += 1;
            }

            if !works {
                start_index = start_index.wrapping_add(1);
                continue 'outer;
            }
            

            for i in start_index..(start_index + buffer.len()) {
                code.remove(i);
            }
            if let Some(new_content) = buffer.iter().map(|pt| pt.content.clone()).reduce(|acc, i| acc + &i) {
                let line = buffer[0].line;
                let char = buffer[0].char;
                let file = buffer[0].file;
                let pt = ParseToken {
                    content: new_content,
                    children: buffer,
                    tags: self.tags.clone(),
                    line,
                    char,
                    file
                };

                if verbose {
                    print!("Creating new ParseToken: {}", &pt);
                }

                code.insert(start_index, pt);

                *changed = true;

                start_index = start_index.wrapping_sub(1);
            }


            start_index = start_index.wrapping_add(1);
        }
    }

    pub fn traverse_and_add_all<'a>(&'a self, code: &mut Vec<ParseToken<'a>>, changed: &mut bool, verbose: bool) {
        let mut start_index: usize = 0;

        'outer: while start_index < code.len() {
            let mut buffer: Vec<usize> = vec![];
            let mut works = true;
            let mut r_index = 0;
            let mut p_index = 0;

            while r_index < self.matches.len() {
                if p_index + start_index >= code.len() {
                    if r_index != self.matches.len() - 1 || Some(r_index) != self.repeat {
                        works = false;
                    }
                    break;
                }
                if !code[p_index + start_index].tags.contains(&self.matches[r_index]) {
                    if Some(r_index) != self.repeat {
                        works = false;
                        break;
                    }
                    else {
                        r_index += 1;
                        if r_index >= self.matches.len() {
                            break;
                        }
                        continue;
                    }
                }
                buffer.push(p_index + start_index);
                if Some(r_index) != self.repeat {
                    r_index += 1;
                }

                p_index += 1;
            }

            if !works {
                start_index = start_index.wrapping_add(1);
                continue 'outer;
            }
            
            for pt in buffer {
                for t in &self.tags {
                    if !code[pt].tags.contains(t) {
                        if verbose {
                            println!("Adding {0:?} to {1}", self.tags, pt);
                        }

                        code[pt].tags.push(t);
                        *changed = true;

                        if verbose {
                            println!("After adding: {}", pt);
                        }
                    }
                }
            }

            start_index += 1;
        }
    }
}