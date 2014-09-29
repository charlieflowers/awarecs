#![feature(globs)]
extern crate the_project;

use the_project::chomp::*;
use the_project::lex::*;

fn main() {
    println!("hello from the lex executable");

    let cr = Some(ChompResult {
        span: Span {
            startPos: Position { index: 42, lineNo: 42, colNo: 42 },
            endPos: Position { index: 44, lineNo: 44, colNo: 44 }
        },
        hitEof: false});

    let token = Number.assert_at(cr);
    // crf!(token);
    assert_eq!(token.tag, Number);
    assert_eq!(token.span.startPos.index, 42);
    assert_eq!(token.span.endPos.index, 44);
    println!("The token is {}", token);
}
