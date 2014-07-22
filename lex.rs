extern crate std; // Don't know why on earth I need this line based on Rust docs, but I do.

use std::iter;
use std::io;
use std::str;
use std::string;

pub struct Lexer {
    meaningOfLife: uint
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
    pub startingIndex: uint,
    pub endingIndex: uint,
    pub text: String
}

impl<'ti> Token<'ti> {
    pub fn make<'ti>(code: &'ti str, tag: TokenTag, startingIndex: uint, endingIndex: uint) -> Token<'ti> {
        let slice = code.slice(startingIndex, endingIndex);
        Token {tag:tag, value: slice, startingIndex: startingIndex,
               endingIndex: endingIndex, text: ("[".to_string() + tag.to_string() + " " + slice.to_string() + "]").to_string()}
    }
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {meaningOfLife: 42}
    }

    // TODO When you refactor lexer to hold on to the string it is lexing, remove all these fn lifetimes and replace with
    //  one single impl lifetime (similar to how you did with Chomper).
    pub fn lex<'fnlex>(&self, code:&'fnlex str) -> Vec<Token<'fnlex>> {
        let index : &mut uint = &mut 0;
        let mut tokens : Vec<Token> = vec![];

        loop {
            if *index >= code.len()  { break; }
            let next_char = code.char_at(*index);
            println!("char {} is {}.", next_char, *index);

            let token = match next_char {
                ws if ws.is_whitespace() => get_whitespace(code, index),
                num if num.is_digit() => get_number(code, index),
                '+' | '-' => get_operator(code, index),
                '#' => get_comment(code, index),
                _ => {fail!("unable to match char {} at index {}", next_char, *index)}
            };

            tokens.push(token);
        }

        tokens
    }
}

fn get_whitespace<'a>(string_contents : &'a str, index : &mut uint) -> Token<'a> {
    let startIndex = *index;
    let mut value = "".to_string();
    loop {
        if *index >= string_contents.len() {break};
        let ch = string_contents.char_at(*index);
        if ! ch.is_whitespace() {
            break;
        } else {
            value = value + std::str::from_char(ch);
            *index = *index+1
        }
    }

    if value.len() == 0  { fail!("You are not supposed to call get_whitespace unless you know you got some. But I found zero characters of whitespace.")}

    return Token::make(string_contents, Whitespace, startIndex, *index);
}

fn get_number<'a>(string_contents : &'a str, index : &mut uint) -> Token<'a> {
    let mut value = "".to_string();
    let startIndex = *index;
    loop {
        let ch = string_contents.char_at(*index);
        if ch.is_digit() {
            value = value + std::str::from_char(ch);
            *index = *index + 1;
        }
        if ! ch.is_digit() || *index >= string_contents.len()  { return Token::make(string_contents, Number, startIndex, *index); }
    }
}

fn get_operator<'a>(string_contents : &'a str, index : &mut uint) -> Token<'a> {
    let mut result = "".to_string();
    let startIndex = *index;
    loop {
        let ch = string_contents.char_at(*index);
        if ch != '+' && ch != '-' { return Token::make(string_contents, Operator, startIndex, *index); }
        result = result + std::str::from_char(ch);
        *index = *index + 1;
        if *index >= string_contents.len() { fail!("Inside get_operator, we ran past end of parser input and were planning to keep going.");}
    }
}

fn get_comment<'a>(string_contents : &'a str, index : &mut uint) -> Token<'a> {
    let startIndex = *index;
    let first_ch = string_contents.char_at(*index);
    if first_ch != '#' { fail!("I thought I was parsing a comment, but it starts with this: {}", first_ch)}
    let next_ch = string_contents.char_at(*index + 1);
    if next_ch == '#' { return get_herecomment(string_contents, index); }
    let mut result = "".to_string();
    loop {
        let ch = string_contents.char_at(*index);
        result = result + std::str::from_char(ch);
        *index = *index + 1;
        if ch == '\n' { return  Token::make(string_contents, Comment, startIndex, *index);}
        if *index >= string_contents.len() { fail!("Inside get_comment, we ran past end of parser input and were planning to keep going.");}
    }
}

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

    pub struct ChompResult<'cr> {
        pub value: &'cr str,
        pub startIndex: uint,
        pub endIndex: uint
    }

    pub struct Chomper<'chomper> {
        code: &'chomper str,
        index: uint,
        char_iterator: Enumerate<Chars<'chomper>>,
        isEof: bool,
    }

    impl<'ci> Chomper<'ci> {
        pub fn new(code: &'ci str) -> Chomper<'ci> {
            Chomper{code: code, index: 0, char_iterator: code.chars().enumerate(), isEof: false}
        }

        fn assert_not_eof(&self) {
            if self.isEof {fail!("Chomper is at EOF."); }
        }

        pub fn next(&mut self) -> Option<(uint, char)> {
            self.assert_not_eof();
            let result = self.char_iterator.next();
            if result == None { self.isEof = true; }
            return result;
        }

        #[inline]
        pub fn chomp(&mut self, quit: |char| -> bool) -> ChompResult<'ci> {
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
                    return ChompResult{ value: self.code.slice(startIndex.unwrap(), endIndex.unwrap()),
                                        startIndex:startIndex.unwrap(), endIndex: endIndex.unwrap() };
                }
            }
        }
    }
}
