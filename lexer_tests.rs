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
    let mut lexer = get_lexer(code);
    let tokens = lexer.lex();
    assertTokensMatch(&tokens, vec!["[Number 40]", "[Whitespace  ]", "[Operator +]", "[Whitespace  ]", "[Number 2]"]);
}

fn dumpTokensToConsole(tokens: Vec<Token> ) {
    let mut index :uint = 1;
    for t in tokens.iter() {
        println!("Token {} is {}", index, t.text);
        index = index + 1;
    }
}

#[test]
fn formula_with_no_spaces_should_succeed() {
    let code = r#"40+2
"#;
    let mut lexer = get_lexer(code);
    let tokens = lexer.lex();
    assertTokensMatch(&tokens, vec!["[Number 40]", "[Operator +]", "[Number 2]"]);
}

#[test]
fn assertTokensMatch_happy_path() {
    let code = "40+2";
    let myTokens = vec![
        Token::make(code, Number, 0, 2),
        Token::make(code, Operator, 2, 3),
        Token::make(code, Number, 3, 4)];

    assertTokensMatch(&myTokens, vec!["[Number 40]", "[Operator +]", "[Number 2]"]);
}

#[test]
#[should_fail]
fn assertTokensMatch_should_fail() {
    let code = "40+2";
    let myTokens = vec![
        Token::make(code, Number, 0, 2),
        Token::make(code, Operator, 2, 3),
        Token::make(code, Number, 3, 4)];

    assertTokensMatch(&myTokens, vec!["[WrongStuff +]"]);
}

#[test]
fn should_handle_number_against_eof() {
    let code = r#"40+2"#;
    let mut lexer = get_lexer(code);
    assertTokensMatch(&lexer.lex(), vec!["[Number 40]", "[Operator +]", "[Number 2]"]);
}

// /////////////////////////////////////////////////////////////////////////////////////////////////

fn get_lexer<'code>(code: &'code str) -> Lexer<'code> {
    Lexer::new(code)
}

fn assertTokensMatch(actualTokens: &Vec<Token>, expectations: Vec<&'static str>) {
    let mut index = 0;
    let mut actualIter = actualTokens.iter();
    for expect in expectations.iter() {
        let token = actualIter.idx(index).unwrap();
        assert_eq!(token.text, expect.to_owned());
        index = index + 1;
    }
}
