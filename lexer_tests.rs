use lex::{Lexer, Token, Number, Operator};
mod lex;

#[test]
fn some_test() {
    println!("All good!");
}

#[test]
fn hello_lex() {
    let code = r#"40 + 2
"#;
    let lexer = get_lexer();
    let tokens = lexer.lex(code);
    assertTokensMatch(tokens, vec!["[Number 40]", "[Whitespace  ]", "[Operator +]", "[Whitespace  ]", "[Number 2]"]);
}

#[test]
fn formula_with_no_spaces_should_succeed() {
    let code = r#"40+2
"#;
    let lexer = get_lexer();
    let tokens = lexer.lex(code);
    assertTokensMatch(tokens, vec!["[Number 40]", "[Operator +]", "[Number 2]"]);
}

#[test]
fn assertTokensMatch_happy_path() {
    let myTokens = vec![
        Token::make(Number, "40".to_owned(), 77),
        Token::make(Operator, "+".to_owned(), 77),
        Token::make(Number, "2".to_owned(), 77)];

    assertTokensMatch(myTokens, vec!["[Number 40]", "[Operator +]", "[Number 2]"]);
}

#[test]
#[should_fail]
fn assertTokensMatch_should_fail() {
    let myTokens = vec![
        Token::make(Number, "40".to_owned(), 77),
        Token::make(Operator, "+".to_owned(), 77),
        Token::make(Number, "2".to_owned(), 77)];

    assertTokensMatch(myTokens, vec!["[WrongStuff +]"]);
}

#[test]
fn should_handle_number_against_eof() {
    let code = r#"40+2"#;
    let lexer = get_lexer();
    assertTokensMatch(lexer.lex(code), vec!["[Number 40]", "[Operator +]", "[Number 2]"]);
}
// /////////////////////////////////////////////////////////////////////////////////////////////////

fn get_lexer() -> Lexer {
    Lexer::new()
}

fn assertTokensMatch(actualTokens: Vec<Token>, expectations: Vec<&'static str>) {
    let mut index = 0;
    let mut actualIter = actualTokens.iter();
    for expect in expectations.iter() {
        let token = actualIter.idx(index).unwrap();
        assert_eq!(token.text, expect.to_owned());
        index = index + 1;
    }
}
