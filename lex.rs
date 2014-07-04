extern crate std; // Don't know why on earth I need this line based on Rust docs, but I do.

use std::iter;
use std::io;
use std::str;

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
pub struct Token<'eddie> {
    pub tag: TokenTag,
    pub value: &'eddie str,
    pub startingIndex: uint,
    pub endingIndex: uint,
    pub text: ~str
}

impl<'eddie> Token<'eddie> {
    // fn new(tag: TokenTag, value: ~str, length: uint, index: uint) -> Token {
    //     Token {tag:tag, value:value, length:length, index:index}
    // }
    pub fn make<'eddie>(code: &'eddie str, tag: TokenTag, startingIndex: uint, endingIndex: uint) -> Token<'eddie> {
        // let fucking_len = value.len();
        // let fucking_index = endingIndex - fucking_len;

        // Token {tag:tag, value:value.to_owned(), length: fucking_len, index: fucking_index,
        //        text: "[".to_owned() + tag.to_str().to_owned() + " ".to_owned() + value.to_owned() + "]".to_owned()}
        let slice = code.slice(startingIndex, endingIndex);
        Token {tag:tag, value: slice, startingIndex: startingIndex,
               endingIndex: endingIndex, text: ("[" + tag.to_str() + " " + slice.to_owned() + "]").to_owned()}
    }
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {meaningOfLife: 42}
    }

    pub fn lex<'a>(&self, code:&'a str) -> Vec<Token<'a>> {
        // let file_bytes = File::open(&Path::new("charlie-to-parse.txt")).read_to_end().unwrap();

        // let contents = file_bytes.as_slice();
        // println!("The slice is {}", contents);

        // let string_contents = str::from_utf8(contents).unwrap();

        let index : &mut uint = &mut 0;
        let mut tokens : Vec<Token> = vec![];

        loop {
            if *index >= code.len()  { break; }
            let next_char = code[*index] as char;
            println!("char {} is {}.", next_char, *index);

            let token = match next_char {
                ws if ws.is_whitespace() => get_whitespace(code, index),
                num if num.is_digit() => get_number(code, index),
                '+' | '-' => get_operator(code, index),
                '#' => get_comment(code, index),
                _ => {fail!("unable to match char {} at index {}", next_char, *index)}
            };

            // println!("Got token: {}", token);
            // println!("After getting token, index is {}", *index);
            tokens.push(token);
        }

        tokens
    }
}

fn get_whitespace<'a>(string_contents : &'a str, index : &mut uint) -> Token<'a> {
    let startIndex = *index;
    let mut value = "".to_owned();
    loop {
        if *index >= string_contents.len() {break};
        let ch = string_contents[*index] as char;
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
    let mut value = "".to_owned();
    let startIndex = *index;
    loop {
        let ch = string_contents[*index] as char;
        if ch.is_digit() {
            value = value + std::str::from_char(ch);
            *index = *index + 1;
        }
        if ! ch.is_digit() || *index >= string_contents.len()  { return Token::make(string_contents, Number, startIndex, *index); }
    }
}

fn get_operator<'a>(string_contents : &'a str, index : &mut uint) -> Token<'a> {
    let mut result = "".to_owned();
    let startIndex = *index;
    loop {
        let ch = string_contents[*index] as char;
        if ch != '+' && ch != '-' { return Token::make(string_contents, Operator, startIndex, *index); }
        result = result + std::str::from_char(ch);
        *index = *index + 1;
        if *index >= string_contents.len() { fail!("Inside get_operator, we ran past end of parser input and were planning to keep going.");}
    }
}

fn get_comment<'a>(string_contents : &'a str, index : &mut uint) -> Token<'a> {
    let startIndex = *index;
    let first_ch = string_contents[*index] as char;
    if first_ch != '#' { fail!("I thought I was parsing a comment, but it starts with this: {}", first_ch)}
    let next_ch = string_contents[*index + 1] as char;
    if next_ch == '#' { return get_herecomment(string_contents, index); }
    let mut result = "".to_owned();
    loop {
        let ch = string_contents[*index] as char;
        result = result + std::str::from_char(ch);
        *index = *index + 1;
        if ch == '\n' { return  Token::make(string_contents, Comment, startIndex, *index);}
        if *index >= string_contents.len() { fail!("Inside get_comment, we ran past end of parser input and were planning to keep going.");}
    }
}

fn expect(string_contents : &str, index : &mut uint, expectation : &str) -> ~str {
    let my_slice = string_contents.slice_from(*index);
    // println!("expecting! At index {}, expecting {} from {}.", *index, expectation, my_slice);

    if ! my_slice.starts_with(expectation) { fail!("At index {}, expected {} but got \r\n {}.", *index, expectation, string_contents.slice_from(*index))}

    let mut result = "".to_owned();
    // todo charlie of course its crazy to append char by char
    for n in range(0, expectation.len()) {
        let actual = my_slice[n] as char;
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
        let ch = string_contents[*index] as char;
        result = result + std::str::from_char(ch);
        if ch == '#' {
            if string_contents[*index + 1] as char == '#' && string_contents[*index + 2] as char == '#' {
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

    pub struct ChompResult<'lt> {
        pub value: &'lt str,
        pub startIndex: uint,
        pub endIndex: uint
    }

    pub struct Chomper<'lt> {
        code: &'lt str,
        index: uint,
        char_iterator: Enumerate<Chars<'lt>>,
    }

    impl<'lt> Chomper<'lt> {
        pub fn new<'lt>(code: &'lt str) -> Box<Chomper<'lt>> {
            box Chomper{code: code, index: 0, char_iterator: code.chars().enumerate()}
        }

        pub fn isEof<'lt>(&'lt self) -> bool {
            self.index >= self.code.len()
        }

        fn assertNotEof<'lt>(&'lt self) {
            if self.isEof() {fail!("Chomper is at EOF."); }
        }

        // pub fn chompTillOne<'lt>(mut self, quit: |char| -> bool) -> ChompResult<'lt> {
        //     self.assertNotEof();
        //     let startIndex = self.index;
        //     loop {
        //         if self.index == self.code.len() || quit(self.code[self.index] as char) {
        //             return ChompResult{ value: self.code.slice(startIndex, self.index), startIndex:startIndex, endIndex: self.index };
        //         }
        //         self.index = self.index + 1;
        //     }
        // }

        // pub fn chompTill<'lt>(&'lt mut self, quit: |char| -> bool) -> ChompResult<'lt> {
        //     self.assertNotEof();
        //     let startIndex = self.index;
        //     loop {
        //         if self.index == self.code.len() || quit(self.code[self.index] as char) {
        //             return ChompResult{ value: self.code.slice(startIndex, self.index), startIndex:startIndex, endIndex: self.index };
        //         }
        //         self.index = self.index + 1;
        //     }
        // }

        pub fn chompTill<'lt>(&'lt mut self, quit: |char| -> bool) -> ChompResult<'lt> {
            self.assertNotEof();
            let mut startIndex: Option<uint> = None;
            let mut endIndex: Option<uint> = None;

            loop {
                let should_quit = match self.char_iterator.next() {
                    None => true,
                    Some((i, ch)) => {
                        if(startIndex == None) { startIndex = Some(i);}
                        endIndex = Some(i);
                        quit (ch)
                    }
                };

                if should_quit {
                    return ChompResult{ value: self.code.slice(startIndex.unwrap(), endIndex.unwrap()),
                                        startIndex:startIndex.unwrap(), endIndex: endIndex.unwrap() };
                }

                // Of course, we have to figure out a plan for the index, todo charlie
            }
        }
    }
}
