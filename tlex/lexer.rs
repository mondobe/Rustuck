#![allow(dead_code)]

use std::{collections::HashMap};

use super::token::*;

pub struct Lexer<'a> {
    pub rules: &'a Vec<Routine<'a>>,
}

impl Lexer<'_> {
    pub fn lex<'a>(&'a self, code: &mut Vec<Token<'a>>, verbose: bool) {
        for rule in self.rules {
            if verbose {
                println!("Starting next rule!");
            }
            rule.start(code, verbose);
        }
    }
}

#[derive(Debug)]
pub struct Routine<'r> {
    pub name: &'r str,
    pub instrs: Vec<Instruction<'r>>
}

impl Routine<'_> {
    pub fn start<'a>(&'a self, code: &mut Vec<Token<'a>>, verbose: bool) {
        let mut machine : Ltm = Ltm {
            index: 0,
            start_index: 0,
            rule_index: 0,
            keep_going: true
        };
        let map: &mut HashMap<&str, usize> = &mut HashMap::new();
        while machine.keep_going { 
            machine.cycle(code, &self.instrs, map, verbose);
        }
    }
}

struct Ltm { // Lexer Turing Machine
    index: usize,
    start_index: usize,
    rule_index: usize,
    keep_going: bool
}

impl Ltm {
    // returns: whether or not to continue
    fn step<'a>(&mut self, code: &mut Vec<Token<'a>>, instrs: &Vec<Instruction<'a>>, labels: &mut HashMap<&'a str, usize>, verbose: bool) {
        let instr = &instrs[self.rule_index];
        let tok = &mut code[self.index];

        match instr {
            Instruction::Next => {
                self.rule_index += 1;
                self.index += 1;
                if verbose {
                    println!("Switching to the next token ({}) and the next rule.", code[self.index]);
                }
            }
            Instruction::Skip => {
                self.rule_index += 1;
                if verbose {
                    println!("Skipping to the next rule.");
                }
            }
            Instruction::Back => {
                self.index = self.index.saturating_sub(1);
                self.rule_index += 1;
                if verbose {
                    println!("Going back to the last token ({}).", code[self.index])
                }
            }
            Instruction::Add(tag) => {
                tok.tags.push(tag);
                self.rule_index += 1;
                if verbose {
                    println!("Adding the tag \"{0}\" to the token {1}.", tag, code[self.index]);
                }
            }
            Instruction::Block(inside) => {
                let mut machine : Ltm = Ltm {
                    index: self.index,
                    start_index: self.start_index,
                    rule_index: 0,
                    keep_going: true
                };
                let map: &mut HashMap<&str, usize> = &mut HashMap::new();
                while machine.keep_going { 
                    machine.step(code, inside, map, verbose);
                }
                self.rule_index += 1;
            }
            Instruction::Delete => {
                if verbose {
                    println!("Deleting the token {}", code[self.index]);
                }
                code.remove(self.index);
                if self.index == self.start_index {
                    self.start_index = self.start_index.wrapping_sub(1);
                }
                self.index = self.index.wrapping_sub(1);
                self.rule_index += 1;
            }
            Instruction::Wrap => {
                let mut new_content = code[self.start_index].content.to_owned();
                for _ in (self.start_index + 1)..self.index {
                    new_content = new_content.to_owned() + &code[self.start_index + 1].content;
                    code.remove(self.start_index + 1);
                }

                code[self.start_index] = Token {
                    content: new_content.clone(),
                    tags: vec![],
                    ..code[self.start_index]
                };
                self.index = self.start_index + 1;
                self.rule_index += 1;

                if verbose {
                    println!("Wrapping all the previous tokens into {}.", new_content.clone());
                }
            }
            Instruction::If(cond) => {
                if code[self.index].tags.contains(cond) {
                    self.rule_index += 1;
                    if verbose {
                        println!("Condition satisfied (tag {0} found on token {1}).", cond, code[self.index]);
                    }
                }
                else if instrs.get(self.rule_index + 2) == Some(&Instruction::Else) {
                    self.rule_index += 3;
                    if verbose {
                        println!("Condition NOT satisfied (tag {0} not found on token {1}), moving to Else clause", 
                        cond, code[self.index]);
                    }
                }
                else {
                    self.rule_index += 2;
                    if verbose {
                        println!("Condition NOT satisfied (tag {0} not found on token {1}).", cond, code[self.index]);
                    }
                }
            }
            Instruction::Cancel => {
                self.keep_going = false;
                if verbose {
                    println!("Cancelling cycle.");
                }
                return;
            }
            Instruction::Else => self.rule_index += 2,
            Instruction::Label(label) => {
                labels.insert(label, self.rule_index);
                self.rule_index += 1;
                if verbose {
                    println!("Label created: {}.", label);
                }
            }
            Instruction::Goto(label) => {
                self.rule_index = labels[label] + 1;
                if verbose {
                    println!("Went to label {}.", label);
                }
            }
            // _ => panic!("Unknown instruction type called! ({:?})", instr)
        }

        self.keep_going = self.rule_index < instrs.len() && self.index < code.len();
    }

    fn cycle<'a>(&mut self, code: &mut Vec<Token<'a>>, instrs: &Vec<Instruction<'a>>, labels: &mut HashMap<&'a str, usize>, verbose: bool) {
        while self.keep_going {
            self.step(code, instrs, labels, verbose);
        }
        
        if verbose {
            println!();
        }
        self.rule_index = 0;
        self.start_index = self.start_index.wrapping_add(1);
        self.index = self.start_index;
        self.keep_going = self.rule_index < instrs.len() && self.index < code.len();
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction<'a> {
    Block(Vec<Instruction<'a>>),
    Next,
    If(&'a str),
    Else,
    Cancel,
    Skip,
    Back,
    Wrap,
    Delete,
    Add(&'a str),
    Label(&'a str),
    Goto(&'a str)
}