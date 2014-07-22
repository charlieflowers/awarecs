use lex::{Lexer, Token, Number, Operator};
use std::iter;
use std::str;

mod lex;

#[test]
fn should_be_able_to_instantiate_chomper() {
    let code = "40 + 2";
    lex::chomp::Chomper::new(code);
}

#[test]
fn chomp_should_work_correctly_when_not_hitting_eof() {
    let code = "40 + 2";
    let mut chomper = lex::chomp::Chomper::new(code);

    let result = chomper.chomp(|ch| { ! ch.is_digit() });

    assert_eq!(result.value, "40");
}

#[test]
fn chomp_should_work_correctly_when_hitting_eof() {
    let code = "40";
    let mut chomper = lex::chomp::Chomper::new(code);

    let result = chomper.chomp(|ch| {
        println!("Seeing if {} is a digit.", ch);
        ! ch.is_digit()
    });

    println!("result is: {}", result);

    assert_eq!(result.value, "40");
}

#[test]
fn chomp_should_succeed_at_2_tokens_in_a_row() {
    let code = "40+2";
    let mut chomper = lex::chomp::Chomper::new(code);

    let one = chomper.chomp(|c| ! c.is_digit());
    assert_eq!(one.value, "40");

    let two = chomper.chomp(|c| c != '+');
    assert_eq!(two.value, "+");
}

#[test]
#[should_fail]
fn chomp_should_return_none_if_youre_already_at_eof_when_you_call_it() {
    let code = "40";
    let mut chomper = lex::chomp::Chomper::new(code);

    let chomper_borrow = &mut chomper;

    let result = chomper_borrow.chomp (|_| { false});
    assert_eq!(result.value, "40");

    chomper_borrow.chomp(|_| { false });
}
