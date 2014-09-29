#![feature(globs)]
extern crate awarecs;

use awarecs::chomp::*;
use awarecs::lex::*;

fn main() {
    println!("hello from the lex executable");

    let cr = Some(ChompResult {
        span: Span {
            start_pos: Position { index: 42, line_no: 42, col_no: 42 },
            end_pos: Position { index: 44, line_no: 44, col_no: 44 }
        },
        hit_eof: false});

    let token = Number.assert_at(cr);
    // crf!(token);
    assert_eq!(token.tag, Number);
    assert_eq!(token.span.start_pos.index, 42);
    assert_eq!(token.span.end_pos.index, 44);
    println!("The token is {}", token);
}
