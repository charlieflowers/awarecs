use std::str::{Chars};
use std::iter::{Enumerate};

#[deriving(Show)]
struct ConsumeResult<'code_to_scan> {
     value: &'code_to_scan str,
     start_index: uint,
     end_index: uint,
}

struct Scanner<'code_to_scan> {
    code: &'code_to_scan str,
    char_iterator: Enumerate<Chars<'code_to_scan>>,
    is_eof: bool,
}

impl<'code_to_scan> Scanner<'code_to_scan> {
    fn new<'code_to_scan>(code: &'code_to_scan str) -> Scanner<'code_to_scan> {
        Scanner{code: code, char_iterator: code.chars().enumerate(), is_eof: false}
    }

    fn assert_not_eof<'code_to_scan>(&'code_to_scan self) {
        if self.is_eof {fail!("Scanner is at EOF."); }
    }

    fn next(&mut self) -> Option<(uint, char)> {
        self.assert_not_eof();
        let result = self.char_iterator.next();
        if result == None { self.is_eof = true; }
        return result;
    }

    // the following line has a problem WHERE THE ARROWS ARE! Interesting, you don't need the lifetime in brackets, cuz it is elsewhere!
    // fn consume_till<'code_to_scan>(&mut self, quit: |char| -> bool) -> ConsumeResult<'code_to_scan> {
    // ...............^^^^^^^^^^^^^^^
    fn consume_till(&mut self, quit: |char| -> bool) -> ConsumeResult<'code_to_scan> {
        self.assert_not_eof();
        let mut start_index: Option<uint> = None;
        let mut end_index: Option<uint> = None;

        loop {
            let should_quit = match self.next() {
                None => {
                    end_index = Some(end_index.unwrap() + 1);
                    true
                },
                Some((i, ch)) => {
                    if start_index == None { start_index = Some(i);}
                    end_index = Some(i);
                    quit (ch)
                }
            };

            if should_quit {
                return ConsumeResult{ value: self.code.slice(start_index.unwrap(), end_index.unwrap()),
                                      start_index:start_index.unwrap(), end_index: end_index.unwrap() };
            }
        }
    }
}

fn main() {
    let test = "this is a string";
    let mut iterator = test.chars();

    iterator.next();
    iterator.next();


    let code_to_scan = "40 + 2";
    let mut scanner = Scanner::new(code_to_scan);
    let first_token = scanner.consume_till(|c| { ! c.is_digit ()});
    println!("first token is: {}", first_token);
    let second_token = scanner.consume_till(|c| { c.is_whitespace ()});
    println!("second token is: {}", second_token);
}
