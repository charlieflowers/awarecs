use lex::{Lexer, Token};
mod lex;

#[test]
fn some_test() {
    println!("All good!");
}

#[test]
fn hello_lex() {
    let code = r#"40 + 2"#;
    let lexer = get_lexer();
    let tokens = lexer.lex(code);
    assertTokensMatch(tokens, vec!["Number 40", "Operator +", "Number 2"]);
}

fn get_lexer() -> Lexer {
    Lexer::new()
}

fn assertTokensMatch(actualTokens: Vec<Token>, expectations: Vec<&'static str>) {
    fail!("not implemented yet");
}
