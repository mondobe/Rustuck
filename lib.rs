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

macro_rules! rule {
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::Instruction::*;

    const INPUT_TEXT: &str = "
    0 1 2 3 4 5 6 -7 8 9 10 11 12
    100 -1000 5000 67.89
    01 0001 -05 0.0
    ";

    #[test]
    fn simple_lexer() {
        let lex = lexer!(
            rule!(
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
            rule!(
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
            rule!(
                :zeroInts=
                    If("0") Do!(
                        Add("int")
                        Add("posInt")
                    )
            )
            rule!(
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
            rule!(
                :decimal=
                    If("int") Next Else Cancel
                    If(".") Next Else Cancel
                    If("int") Next Else Cancel
                    Wrap
                    Back
                    Add("decimal")
            )
            rule!(
                :noWs=
                    If("ws")
                        Delete
            )
        );

        let code = &mut to_tokens(INPUT_TEXT, "input");
        lex.lex(code, false);
        print_tokens(code);
    }

    #[test]
    fn output_token_stream() {
        print_tokens(&to_tokens("Hello, world!", "input"));
    }
}
