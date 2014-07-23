extern crate std; // Don't know why on earth I need this line based on Rust docs, but I do.

pub struct Lexer<'lexer> {
    chomper: chomp::Chomper<'lexer>,
}

#[deriving(Show)]
pub enum TokenTag {
    Number,
    Whitespace,
    Operator,
    Herecomment,
    Comment
}

#[deriving(Show)]
pub struct Token<'token> {
    pub tag: TokenTag,
    pub value: &'token str,
    pub startIndex: uint,
    pub endIndex: uint,
    pub text: String
}

impl<'ti> Token<'ti> {
    pub fn make<'ti>(my_slice: &'ti str, tag: TokenTag, startIndex: uint, endIndex: uint) -> Token<'ti> {
        // todo get rid of the "text" field because it of course copies the whole source code & you worked so hard to avoid copies
        Token {tag:tag, value: my_slice, startIndex: startIndex,
               endIndex: endIndex, text: ("[".to_string() + tag.to_string() + " " + my_slice.to_string() + "]").to_string()}
    }
}

impl<'li> Lexer<'li> {
    pub fn new(code: &'li str) -> Lexer<'li> {
        Lexer {chomper: chomp::Chomper::new(code)}
    }

    pub fn lex(&mut self) -> Vec<Token<'li>> {
        let index : &mut uint = &mut 0;
        let mut tokens : Vec<Token> = vec![];

        loop {
            if self.chomper.isEof { break; }
            let next_char_o = self.chomper.peek();
            if next_char_o == None { break; } // todo ugliness
            let next_char = next_char_o.unwrap();

            let token = match next_char {
                ws if ws.is_whitespace() => self.get_whitespace(),
                num if num.is_digit() => self.get_number(),
                '+' | '-' => self.get_operator(),
                '#' => self.get_comment(),
                _ => {fail!("Charlie, you have not implemented ability to match char {} at index {}", next_char, *index)}
            };

            println!("Got token!! {}", token);
            println!("Chomper peek char is {}", self.chomper.peek());
            println!("At this point, index is {}", self.chomper.index);

            tokens.push(token);
        }

        tokens
    }

    pub fn get_whitespace(&mut self) -> Token<'li> { // todo, ONLY pub so you can test it, fix that later
        let result = self.chomper.chomp(|ch| { ! ch.is_whitespace() }).unwrap();
        println!("Here's the whitespace chomp result: {}", result);
        if result.is_empty() {fail!("You are not supposed to call get_whitespace unless you know you have some. But no whitespace was found.")}

        Token::make(result.value, Whitespace, result.startIndex, result.endIndex)
    }

    pub fn get_number(&mut self) -> Token<'li> {
        let result = self.chomper.chomp(|c| {! c.is_digit()} ).unwrap();
        Token::make(result.value, Number, result.startIndex, result.endIndex)
    }

    pub fn get_operator(&mut self) -> Token<'li> {
        let result = self.chomper.chomp(|c| {c != '+' && c != '-'}).unwrap();
        Token::make(result.value, Operator, result.startIndex, result.endIndex)
    }

    pub fn get_comment(&mut self) -> Token<'li> {
        // todo next line can be nicer
        if self.chomper.peek() != Some('#') { fail!("I thought I was parsing a comment, but it starts with this: {}", self.chomper.peek())}
        println!("seeing if we have herecomment");

        match self.chomper.text().slice_to(3) { // todo can probably pattern match more gracefully here
            "###" => self.get_here_comment(),
            _ => {
                println!("in get_comment, and decided it was NOT a herecomment.");
                println!("text is: {}", self.chomper.text());
                let result = self.chomper.chomp(|c| {c == '\n'}).unwrap();
                Token::make(result.value, Comment, result.startIndex, result.endIndex)
            }
        }
    }

    pub fn get_here_comment(&mut self) -> Token<'li> {
        self.chomper.expect("###");
        let cr = self.chomper.chomp_till_str(|str| str.starts_with("###")).unwrap();
        let mut endIndex = cr.endIndex;
        if ! self.chomper.isEof {
            self.chomper.expect("###");
            endIndex = cr.endIndex + 3;
        }
        Token::make(self.chomper.code.slice(cr.startIndex - 3, endIndex), Herecomment, cr.startIndex - 3, endIndex)
    }
}

pub mod chomp {
    pub use std::str::{Chars};
    pub use std::iter::{Enumerate};

    #[deriving(Show)]
    pub struct ChompResult<'cr> {
        pub value: &'cr str,
        pub startIndex: uint,
        pub endIndex: uint
    }

    impl<'cri> ChompResult<'cri> {
        pub fn is_empty(&self) -> bool {
            self.value.len() == 0
        }
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
}
