# Rustuck

The Rust T-Lex/Utah Construction Kit

Rustuck is a Rust crate offering a set of powerful utilities for lexing and parsing text files. As a clone of the Tuck (TLex/Utah Construction Kit) framework for C#, the design is tried, tested, and set in stone. Rustuck can be used to efficiently create anything from a simple domain-specific data parser to a full-on general purpose language. 

Rustuck works by creating two data structures, each with different rules and mechanics: a lexer and a parser. These structures can be created simply through the `lexer!` and `parser!` macros in Rust, which allow lexers and parsers to be easily defined in a simple, understandable syntax. 

What makes Rustuck's lexer so unique, simple, and powerful is its imperative syntax. Similar to languages like Basic, the imperative syntax defines step-by-step rules to make it entirely clear what the lexer will do:

```
routine!(
	:integers=
	If("nonzero") Next Else Cancel // if a non-zero digit is found, move on
	If("digit") Repeat Else Do!(   // keep going as long as the current token is a digit
		Wrap Back Add("integer")   // create a single token with the tag "integer"
	)
)
routine!(
	:zeroInt=
	If("0") Add("integer")  // if the current character is 0, create an "integer" token
)
```

This code compiles with the rest of your Rust code down to binary, and does not have any external runtime. It is just a specialized language for writing lexers, all inside a Rust macro.

For the parser, a simple declarative syntax is used, with a superset of the Backus-Naur form providing the ease of use of any other mainstream parser generator with, once again, native speeds.

```
rule!("integer" ;; "expression") // classify all integers as expressions
rule!("expression" "operator" "expression" ; "expression") // arithmetic expressions
rule!("let" "name" "eq" "expression" ; "statement") // define variables
rule!("name" ;; "expression") // use variables
rule!("print" "expression" ; "statement") // print expressions
```

These lexers and parsers analyze code, creating `Token`s and `ParseToken`s respectively. These can then be further analyzed within Rust to create an abstract syntax tree, which can itself be compiled or sent to a tool like LLVM.

```
fn main() {
	let input: &str = "...";
	let lexer: Lexer = lexer!(...);
	let parser: Parser = parser!(...);
	let output = lex_and_parse(lexer, parser, input, false); // verbose: false
}
```

