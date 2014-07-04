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
fn chomp_till_should_work_correctly_when_not_hitting_eof() {
    let code = "40 + 2";
    let mut chomper = lex::chomp::Chomper::new(code);

    let result = chomper.chompTill(|ch| { ! ch.is_digit() });

    assert_eq!(result.value, "40");
}

#[test]
fn chomp_till_should_work_correctly_when_hitting_eof() {
    let code = "40";
    let mut chomper = box lex::chomp::Chomper::new(code);

    let result = chomper.chompTill(|ch| {
        println!("Seeing if {} is a digit.", ch);
        ! ch.is_digit()
    });

    assert_eq!(result.value, "40");
}

// #[test]
// fn does_a_struct_with_a_borrowed_ref_to_string_have_move_semantics() {
//     struct has_ref_string<'lt> {
//         text: &'lt str
//     }

//     struct has_owned_string {
//         text: ~str
//     }

//     let one = has_ref_string {text: "This is a test"};
//     let two = one;

//     println!("one is {}", one.text);
//     println!("two is {}", two.text);

//     let three = has_owned_string { text: "This has an owned string".to_owned()};
//     let four = three; // I think this will MOVE four.

//     println!("three is {}", three.text); // this should blow up because we moved out of three.
//     println!("four is {}", four.text);

//     // it did exactly as i expected. So a &str does NOT force move semantics, but an ~str does.
//     // fail!("show me output");
// }

// #[test]
// #[should_fail]
// fn chomp_till_should_return_none_if_youre_already_at_eof_when_you_call_it() {
//     let code = "40";
//     let mut chomper : & mut lex::chomp::Chomper = lex::chomp::Chomper::new(code);

//     // {
//         let result = chomper.chompTill (|ch| { false});
//         assert_eq!(result.value, "40");
//     // }

//     let after_eof = chomper.chompTill(|ch| { false });
// }

#[test]
fn does_chars_iterator_handle_lifetimes_more_tightly() {
    let code = r#"40 + 2"#;

    let mut chars = code.chars();

    println!("First char: {}", chars.next().unwrap());
    println!("Second char: {}", chars.next().unwrap());

    // Yes it does! This works!
}
