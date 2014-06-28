use lex::{Lexer, Token, Number, Operator};
mod lex;

#[test]
fn should_be_able_to_instantiate_chomper() {
    let code = "40 + 2";
    lex::chomp::Chomper::new(code);
}

#[test]
fn chomp_till_should_work_correctly_when_not_hitting_eof() {
    let code = "40 + 2";
    let mut chomper = lex::chomp::Chomper::new(code);

    let result = chomper.chompTill(|ch| { ! ch.is_digit() }).unwrap();

    assert_eq!(result.value, "40");
}

#[test]
fn chomp_till_should_work_correctly_when_hitting_eof() {
    let code = "40";
    let mut chomper = lex::chomp::Chomper::new(code);

    let result = chomper.chompTill(|ch| { ! ch.is_digit() }).unwrap();

    assert_eq!(result.value, "40");
}

#[test]
fn chomp_till_should_return_none_if_youre_already_at_eof_when_you_call_it() {
    let code = "40";
    let mut chomper = lex::chomp::Chomper::new(code);

    let result = chomper.chompTill(|ch| { false}).unwrap();

    assert_eq!(result.value, "40");

    let after_eof = chomper.chompTill(|ch| { false });

    assert_eq!(after_eof, None);
}
