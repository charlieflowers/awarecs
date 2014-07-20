pub use std::str::{Chars};
pub use std::iter::{Enumerate};

#[deriving(Show)]
pub struct ConsumeResult<'lt> {
    pub value: &'lt str,
    pub startIndex: uint,
    pub endIndex: uint,
}

pub struct Scanner<'lt> {
    code: &'lt str,
    char_iterator: Enumerate<Chars<'lt>>,
    isEof: bool,
}

impl<'lt> Scanner<'lt> {
    pub fn new<'lt>(code: &'lt str) -> Scanner<'lt> {
        Scanner{code: code, char_iterator: code.chars().enumerate(), isEof: false}
    }

    fn assert_not_eof<'lt>(&'lt self) {
        if self.isEof {fail!("Scanner is at EOF."); }
    }

    pub fn next(&mut self) -> Option<(uint, char)> {
        self.assert_not_eof();
        let result = self.char_iterator.next();
        if result == None { self.isEof = true; }
        return result;
    }

    pub fn consume_till<'lt>(&'lt mut self, quit: |char| -> bool) -> ConsumeResult<'lt> {
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
    let codeToScan = "40 + 2";
    let mut scanner = Scanner::new(codeToScan);
    let first_token = scanner.consume_till(|c| { ! c.is_digit ()});
    println!("first token is: {}", first_token);
    // scanner.consume_till(|c| { c.is_whitespace ()}); // WHY DOES THIS LINE FAIL?
}
