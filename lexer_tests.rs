use lex::{Lexer, Token, Number, Operator};
mod lex;

#[test]
fn hello_lex() {
    let code = r#"40 + 2
"#;
    let mut lexer = get_lexer(code);
    let tokens = lexer.lex();
    assert_tokens_match(&tokens, vec!["[Number 40]", "[Whitespace  ]", "[Operator +]", "[Whitespace  ]", "[Number 2]"]);
}

fn dump_tokens_to_console(tokens: Vec<Token> ) {
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
    assert_tokens_match(&tokens, vec!["[Number 40]", "[Operator +]", "[Number 2]"]);
}

#[test]
fn make_sure_assert_tokens_itself_works() {
    let code = "40+2";
    let myTokens = vec![
        Token::make("40", Number, 0, 2),
        Token::make("+", Operator, 2, 3),
        Token::make("2", Number, 3, 4)];

    assert_tokens_match(&myTokens, vec!["[Number 40]", "[Operator +]", "[Number 2]"]);
}

#[test]
#[should_fail]
fn make_sure_assert_tokens_fails_when_it_should() {
    let code = "40+2";
    let myTokens = vec![
        Token::make(code, Number, 0, 2),
        Token::make(code, Operator, 2, 3),
        Token::make(code, Number, 3, 4)];

    assert_tokens_match(&myTokens, vec!["[WrongStuff +]"]);
}

#[test]
fn should_handle_number_against_eof() {
    let code = r#"40+2"#;
    let mut lexer = get_lexer(code);
    assert_tokens_match(&lexer.lex(), vec!["[Number 40]", "[Operator +]", "[Number 2]"]);
}

// /////////////////////////////////////////////////////////////////////////////////////////////////

fn get_lexer<'code>(code: &'code str) -> Lexer<'code> {
    Lexer::new(code)
}

fn assert_tokens_match(actualTokens: &Vec<Token>, expectations: Vec<&'static str>) {
    println!("Matching tokens: ");
    println!("   Expecting: {}", expectations);
    println!("   Actual: Len of {}, -> {}", actualTokens.len(), actualTokens);

    let mut index = 0;
    let mut actualIter = actualTokens.iter();
    for expect in expectations.iter() {
        let token = actualIter.idx(index).unwrap();
        assert_eq!(token.text, expect.to_string());
        index = index + 1;
    }
}
