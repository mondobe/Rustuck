/*!
 * Rustuck is an enhanced Rust implementation of TUCK: the T-Lex/Utahparser 
 * construction kit. It has a collection of macros to support easy lexer and
 * parser generation along with a set of functions to execute your imperative
 * lexer and declarative parser code on sets of text or tokens respectively.
 */

#![allow(unused_macros)]
pub mod tlex;
pub mod utah;
#[macro_use]
pub mod macros;

pub use tlex::lexer::*;
pub use tlex::token::*;
pub use utah::parse_token::*;
pub use utah::parser::*;

/// Returns a vector of ParseTokens representing a fully lexed and parsed 
/// string
/// 
/// # Arguments
/// 
/// - lexer - A reference to a lexer that represents the series of routines 
/// that will be executed on the string
/// - parser - A reference to a parser that represents the series of rules that
/// will be executed on the vector of tokens returned from the lexing
/// - code - The string that is input into the lexer
/// - verbose - Whether to print debug information

pub fn lex_and_parse<'a>(
    lexer: &'a Lexer, 
    parser: &'a Parser, 
    code: &'a str, 
    verbose: bool
) -> Vec<ParseToken<'a>> {
    let code = &mut to_tokens(code, "input");
    let lex = lexer;
    lex.lex(code, verbose);
    let parse = parser;
    let mut code = to_parse_tokens(code.to_vec());
    parse.parse(&mut code, verbose);
    code
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Instruction::*;

    macro_rules! number_lexer {
        () => {
            lexer!(
                routine!(
                    :digits=
                        TagFrags!("digit", 
                            "0" "1" "2" "3" "4"
                            "5" "6" "7" "8" "9"
                        ) 
                        TagFrags!("nonzero",
                            "1" "2" "3" "4" "5"
                            "6" "7" "8" "9"
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
            parser!(
                    rule!("int" "int" ; "pair")
            )
        };
    }

    const INPUT_TEXT: &str = "
    0 1 2 3 4 5 6 -7 8 9 10 11 12
    100 -1000 5000 67.89
    01 0001 -05 0.0
    -0.34 6.-98 -9.04
    ";


    #[test]
    fn simple_lexer() {
        print_parse_tokens(lex_and_parse(&number_lexer!(), &pair_parser!(), 
                INPUT_TEXT, false));
    }

    #[test]
    fn output_token_stream() {
        print_tokens(&to_tokens("Hello, world!", "input"));
    }
}
