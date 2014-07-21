use std::str::{Chars};
use std::iter::{Enumerate};

#[deriving(Show)]
struct ConsumeResult<'code_to_scan> {
     value: &'code_to_scan str,
     startIndex: uint,
     endIndex: uint,
}

struct Scanner<'code_to_scan> {
    code: &'code_to_scan str,
    char_iterator: Enumerate<Chars<'code_to_scan>>,
    isEof: bool,
}

impl<'code_to_scan> Scanner<'code_to_scan> {
    fn new<'code_to_scan>(code: &'code_to_scan str) -> Scanner<'code_to_scan> {
        Scanner{code: code, char_iterator: code.chars().enumerate(), isEof: false}
    }

    fn assert_not_eof<'code_to_scan>(&'code_to_scan self) {
        if self.isEof {fail!("Scanner is at EOF."); }
    }

    fn next(&mut self) -> Option<(uint, char)> {
        self.assert_not_eof();
        let result = self.char_iterator.next();
        if result == None { self.isEof = true; }
        return result;
    }

    // the following line has a problem WHERE THE ARROWS ARE! Interesting, you don't need the lifetime in brackets, cuz it is elsewhere!
    // fn consume_till<'code_to_scan>(&mut self, quit: |char| -> bool) -> ConsumeResult<'code_to_scan> {
    // ...............^^^^^^^^^^^^^^^
    fn consume_till(&mut self, quit: |char| -> bool) -> ConsumeResult<'code_to_scan> {
        self.assert_not_eof();
        let mut startIndex: Option<uint> = None;
        let mut endIndex: Option<uint> = None;

        loop {
            let should_quit = match self.next() {
                None => {
                    endIndex = Some(endIndex.unwrap() + 1);
                    true
                },
                Some((i, ch)) => {
                    if startIndex == None { startIndex = Some(i);}
                    endIndex = Some(i);
                    quit (ch)
                }
            };

            if should_quit {
                return ConsumeResult{ value: self.code.slice(startIndex.unwrap(), endIndex.unwrap()),
                                      startIndex:startIndex.unwrap(), endIndex: endIndex.unwrap() };
            }
        }
    }
}

fn main() {
    let test = "this is a string";
    let mut iterator = test.chars();

    iterator.next();
    iterator.next();


    let codeToScan = "40 + 2";
    let mut scanner = Scanner::new(codeToScan);
    let first_token = scanner.consume_till(|c| { ! c.is_digit ()});
    println!("first token is: {}", first_token);
    let second_token = scanner.consume_till(|c| { c.is_whitespace ()});
    println!("second token is: {}", second_token);
}
