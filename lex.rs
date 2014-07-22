extern crate std; // Don't know why on earth I need this line based on Rust docs, but I do.

use std::iter;
use std::io;
use std::str;
use std::string;

pub struct Lexer<'lexer> {
    code: &'lexer str,
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
        Lexer {code: code, chomper: chomp::Chomper::new(code)}
    }

    // TODO When you refactor lexer to hold on to the string it is lexing, remove all these fn lifetimes and replace with
    //  one single impl lifetime (similar to how you did with Chomper).

    pub fn lex(&mut self) -> Vec<Token<'li>> {
        let index : &mut uint = &mut 0;
        let mut tokens : Vec<Token> = vec![];

        loop {
            if self.chomper.isEof { break; }
            let next_char_o = self.chomper.peek();
            if next_char_o == None { break; } // todo ugliness
            let next_char = next_char_o.unwrap();
            // println!("char {} is {}.", next_char, *index);

            let token = match next_char {
                ws if ws.is_whitespace() => self.get_whitespace(),
                num if num.is_digit() => self.get_number(),
                '+' | '-' => self.get_operator(),
                '#' => self.get_comment(),
                _ => {fail!("unable to match char {} at index {}", next_char, *index)}
            };

            println!("Got token!! {}", token);
            println!("Chomper peek char is {}", self.chomper.peek());
            println!("At this point, index is {}", self.chomper.index);

            tokens.push(token);
        }

        tokens
    }

    pub fn get_whitespace(&mut self) -> Token<'li> { // todo, ONLY pub so you can test it, fix that later
        let result = self.chomper.chomp(|ch| { ! ch.is_whitespace() });
        println!("Here's the whitespace chomp result: {}", result);
        if result.is_empty() {fail!("You are not supposed to call get_whitespace unless you know you have some. But no whitespace was found.")}

        Token::make(result.value, Whitespace, result.startIndex, result.endIndex)
    }

    pub fn get_number(&mut self) -> Token<'li> {
        let result = self.chomper.chomp(|c| {! c.is_digit()} );
        Token::make(result.value, Number, result.startIndex, result.endIndex)
    }

    pub fn get_operator(&mut self) -> Token<'li> {
        let result = self.chomper.chomp(|c| {c != '+' && c != '-'});
        Token::make(result.value, Operator, result.startIndex, result.endIndex)
    }

    pub fn get_comment(&mut self) -> Token<'li> {
        // todo next line can be nicer
        if self.chomper.peek() != Some('#') { fail!("I thought I was parsing a comment, but it starts with this: {}", self.chomper.peek())}
        match self.chomper.text().slice_to(1) { // todo can probably pattern match more gracefully here
            "##" => self.get_here_comment(),
            _ => {
                let result = self.chomper.chomp(|c| {c == '\n'});
                Token::make(result.value, Comment, result.startIndex, result.endIndex)
            }
        }
    }

    pub fn get_here_comment(&mut self) -> Token<'li> {
        fail!("Todo, transform get_here_comment from old code")
    }
}

// fn get_whitespace<'a>(string_contents : &'a str, index : &mut uint) -> Token<'a> {
//     let startIndex = *index;
//     let mut value = "".to_string();
//     loop {
//         if *index >= string_contents.len() {break};
//         let ch = string_contents.char_at(*index);
//         if ! ch.is_whitespace() {
//             break;
//         } else {
//             value = value + std::str::from_char(ch);
//             *index = *index+1
//         }
//     }

//     if value.len() == 0  { fail!("You are not supposed to call get_whitespace unless you know you got some. But I found zero characters of whitespace.")}

//     return Token::make(string_contents, Whitespace, startIndex, *index);
// }

// fn get_number<'a>(string_contents : &'a str, index : &mut uint) -> Token<'a> {
//     let mut value = "".to_string();
//     let startIndex = *index;
//     loop {
//         let ch = string_contents.char_at(*index);
//         if ch.is_digit() {
//             value = value + std::str::from_char(ch);
//             *index = *index + 1;
//         }
//         if ! ch.is_digit() || *index >= string_contents.len()  { return Token::make(string_contents, Number, startIndex, *index); }
//     }
// }

// fn get_operator<'a>(string_contents : &'a str, index : &mut uint) -> Token<'a> {
//     let mut result = "".to_string();
//     let startIndex = *index;
//     loop {
//         let ch = string_contents.char_at(*index);
//         if ch != '+' && ch != '-' { return Token::make(string_contents, Operator, startIndex, *index); }
//         result = result + std::str::from_char(ch);
//         *index = *index + 1;
//         if *index >= string_contents.len() { fail!("Inside get_operator, we ran past end of parser input and were planning to keep going.");}
//     }
// }

// fn get_comment<'a>(string_contents : &'a str, index : &mut uint) -> Token<'a> {
//     let startIndex = *index;
//     let first_ch = string_contents.char_at(*index);
//     if first_ch != '#' { fail!("I thought I was parsing a comment, but it starts with this: {}", first_ch)}
//     let next_ch = string_contents.char_at(*index + 1);
//     if next_ch == '#' { return get_herecomment(string_contents, index); }
//     let mut result = "".to_string();
//     loop {
//         let ch = string_contents.char_at(*index);
//         result = result + std::str::from_char(ch);
//         *index = *index + 1;
//         if ch == '\n' { return  Token::make(string_contents, Comment, startIndex, *index);}
//         if *index >= string_contents.len() { fail!("Inside get_comment, we ran past end of parser input and were planning to keep going.");}
//     }
// }

fn expect(string_contents : &str, index : &mut uint, expectation : &str) -> String {
    let my_slice = string_contents.slice_from(*index);
    // println!("expecting! At index {}, expecting {} from {}.", *index, expectation, my_slice);

    if ! my_slice.starts_with(expectation) { fail!("At index {}, expected {} but got \r\n {}.", *index, expectation, string_contents.slice_from(*index))}

    let mut result = "".to_string();
    // todo charlie of course its crazy to append char by char
    for n in range(0, expectation.len()) {
        let actual = my_slice.char_at(n);
        result = result + std::str::from_char(actual);
        *index = *index + 1;
    }
    return result;
}

fn get_herecomment<'a>(string_contents : &'a str, index : &mut uint) -> Token<'a> {
    let startIndex = *index;
    let mut result = expect(string_contents, index, "###");
    // Just keep going no matter what, until you hit the end or find ###.
    loop {
        // println!("in mystery loop, index is {}", *index);
        // println!("the rest of the string is {}", string_contents.slice_from(*index));
        let ch = string_contents.char_at(*index);
        result = result + std::str::from_char(ch);
        if ch == '#' {
            if string_contents.char_at(*index + 1) == '#' && string_contents.char_at(*index + 2) == '#' {
                result = result + expect(string_contents, index, "###");
                // println!("second expect completed, and result is: {}", result);
                return  Token::make(string_contents, Herecomment, startIndex, *index);
            }
        }
        *index = *index + 1;
        if *index >= string_contents.len() { fail!("Inside get_herecomment, we ran past end of parser input and were planning to keep going. The herecomment token we have so far is {}", result);}
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

        #[inline]
        pub fn chomp(&mut self, quit: |char| -> bool) -> ChompResult<'ci> {
            self.assert_not_eof();
            let mut startIndex: Option<uint> = None;
            let mut endIndex: Option<uint> = None;

            // todo I KNOW this can be simplified and cleaned up
            loop {
                let should_quit = match self.peek() {
                    None => {
                        // This means, there IS no next character! EOF!
                        endIndex = Some(self.index);
                        true
                    },
                    Some(ch) => {
                        if quit(ch) {
                            endIndex = Some(self.index);
                            true
                        } else {
                            if startIndex == None { startIndex = Some(self.index);}
                            println!("just about to call self.next");
                            self.next();
                            // if nr == None {
                            //     println!("I DID call self.next, and it was None");
                            //     endIndex = Some(self.index);
                            //     true
                            // } else {
                            false
                        }
                    }
                };

                if should_quit {
                    println!("Just about to create ChompResult");
                    println!("startIndex is: {}", startIndex);
                    println!("endIndex is: {}", endIndex);

                    return ChompResult{ value: self.code.slice(startIndex.unwrap(), endIndex.unwrap()),
                                        startIndex:startIndex.unwrap(), endIndex: endIndex.unwrap() };
                }
            }
        }
    }
}
