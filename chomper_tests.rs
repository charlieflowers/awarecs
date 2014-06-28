use lex::{Lexer, Token, Number, Operator};
mod lex;

#[test]
fn should_be_able_to_instantiate_chomper() {
    let code = "40 + 2";
    lex::chomp::Chomper::new(code);
}
