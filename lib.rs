pub mod tlex;
pub mod utah;

pub use tlex::lexer::*;
pub use tlex::token::*;
pub use utah::parse_token::*;

macro_rules! TagFrags {
    ($tag:expr, $($frag:expr)*) => {
        Block(vec![
            $(
                If($frag), Add($tag),
            )*
        ])
    };
}

macro_rules! Do {
    ($($instr:expr)*) => {
        Block(vec![
            $(
                $instr,
            )*
        ])
    };
}

macro_rules! routine {
    (:$name:ident= $($instr:expr)*) => {
        {
            let rule_instrs : Vec<Instruction> = vec![
                $(
                    $instr,
                )*
            ];

            Routine {
                name: stringify!($name),
                instrs: rule_instrs
            }
        }
    };
}

macro_rules! lexer {
    ($($rule:expr)*) => {
        Lexer {
            rules: &vec![
                $(
                    $rule,
                )*
            ]
        }
    };
}

macro_rules! number_lexer {
    () => {
        lexer!(
            routine!(
                :digits=
                    TagFrags!("digit", 
                        "0"
                        "1"
                        "2"
                        "3"
                        "4"
                        "5"
                        "6"
                        "7"
                        "8"
                        "9"
                    ) 
                    TagFrags!("nonzero",
                        "1"
                        "2"
                        "3"
                        "4"
                        "5"
                        "6"
                        "7"
                        "8"
                        "9"
                    )
            )
            routine!(
                :ints=
                    If("nonzero")
                        Skip
                    Else
                        Cancel
                    Label("Test")
                    Next
                    If("digit")
                        Goto("Test")
                    Do!(
                        Wrap
                        Back
                        Add("int")
                        Add("posInt")
                    )
            )
            routine!(
                :zeroInts=
                    If("0") Do!(
                        Add("int")
                        Add("posInt")
                    )
            )
            routine!(
                :negatives=
                    If("-") Next Else Cancel
                    If("posInt") Do!(
                        Next
                        Wrap
                        Back
                        Add("int")
                        Add("negInt")
                    )
            )
            routine!(
                :decimal=
                    If("int") Next Else Cancel
                    If(".") Next Else Cancel
                    If("posInt") Next Else Cancel
                    Wrap
                    Back
                    Add("decimal")
            )
            routine!(
                :noWs=
                    If("ws")
                        Delete
            )
        )
    };
}

macro_rules! pair_parser {
    () => {
    Parser {
        rules: &vec![
            Rule {
                matches: vec!["int", "int"],
                tags: vec!["pair"],
                repeat: None,
                add_all: false
            }
        ]
    }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Instruction::*;
    use super::utah::parser::*;

    const INPUT_TEXT: &str = "
    0 1 2 3 4 5 6 -7 8 9 10 11 12
    100 -1000 5000 67.89
    01 0001 -05 0.0
    -0.34 6.-98 -9.04
    ";


    #[test]
    fn simple_lexer() {
        let code = &mut to_tokens(INPUT_TEXT, "input");
        let lex = number_lexer!();
        lex.lex(code, true);
        let parse = pair_parser!();
        let mut code = to_parse_tokens(code.to_vec());
        parse.parse(&mut code, true);
        print_parse_tokens(code);
    }

    #[test]
    fn output_token_stream() {
        print_tokens(&to_tokens("Hello, world!", "input"));
    }
}
