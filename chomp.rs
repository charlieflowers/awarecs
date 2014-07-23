extern crate std; // Don't know why on earth I need this line based on Rust docs, but I do.

pub use std::str::{Chars};
pub use std::iter::{Enumerate};

#[deriving(Show)]
pub struct ChompResult<'cr> {
    pub value: &'cr str,
    pub startIndex: uint,
    pub endIndex: uint
}

pub struct Chomper<'chomper> {
    pub code: &'chomper str,
    pub index: uint,
    char_iterator: Enumerate<Chars<'chomper>>,
    pub isEof: bool,
}

impl<'ci> Chomper<'ci> {
    pub fn new(code: &'ci str) -> Chomper<'ci> {
        Chomper{code: code, index: 0, char_iterator: code.chars().enumerate(), isEof: false}
    }

    fn assert_not_eof(&self) {
        if self.isEof {fail!("Chomper is at EOF."); }
    }

    pub fn peek(&self) -> Option<char> {
        let target = self.index;
        if target >= self.code.len() {return None};
        Some(self.code.char_at(target))
    }

    pub fn text(&self) -> &'ci str {
        self.code.slice_from(self.index)
    }
    pub fn next(&mut self) -> Option<(uint, char)> {
        self.assert_not_eof();
        let result = self.char_iterator.next();
        if result == None { self.isEof = true; }
        self.index = self.index + 1;
        return result;
    }

    pub fn expect(&mut self, expectation: &str) -> ChompResult<'ci> {
        if ! self.text().starts_with(expectation) {
            fail!("At index {}, expected {} but got \r\n {}.", self.index, expectation, self.text())
        }

        let mut chomped = 0;

        self.chomp(|_| {
            chomped = chomped + 1;
            chomped > expectation.len()
        }).unwrap()
    }

    pub fn chomp_till_str(&mut self, quit: |&str| -> bool) -> Option<ChompResult<'ci>> {
        self.chomp_internal(|_| false, quit)
    }

    pub fn chomp(&mut self, quit: |char| -> bool) -> Option<ChompResult<'ci>> {
        self.chomp_internal(quit, |_| false)
    }

    fn chomp_internal(&mut self, char_quit: |char| -> bool, str_quit: |&str| -> bool) -> Option<ChompResult<'ci>> {
        self.assert_not_eof();
        let mut startIndex: Option<uint> = None;
        let mut endIndex: Option<uint> = None;

        println!("starting a chomp at text: {}", self.text());
        println!("index is: {}", self.index);
        println!("isEof is {}", self.isEof);
        println!("last valid index of code is {}", self.code.len() - 1);
        // todo I KNOW this can be simplified and cleaned up
        loop {
            let should_quit = match self.peek() {
                None => {
                    // This means, there IS no next character. EOF.
                    endIndex = Some(self.index);
                    // Still need to call next(), to fully put chomper into EOF state.
                    self.next();
                    true
                },
                Some(ch) => {
                    if char_quit(ch) || str_quit(self.text()) {
                        endIndex = Some(self.index);
                        true
                    } else {
                        println!("Not time to quit yet!");
                        if startIndex == None {
                            println!("setting start index for chomp at {}", self.index);
                            startIndex = Some(self.index);
                        }
                        self.next();
                        false
                    }
                }
            };

            if should_quit {
                println!("Just about to create ChompResult");
                println!("startIndex is: {}", startIndex);
                println!("endIndex is: {}", endIndex);

                if startIndex == None {return None;}
                let cr = Some(ChompResult { value: self.code.slice(startIndex.unwrap(), endIndex.unwrap()),
                                            startIndex:startIndex.unwrap(), endIndex: endIndex.unwrap() });

                println!("Full chomp result is: {}", cr);
                return cr;
            }
        }
    }
}
