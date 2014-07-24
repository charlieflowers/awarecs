extern crate std; // Don't know why on earth I need this line based on Rust docs, but I do.

use chomp::{Chomper, ChompResult};

mod chomp;

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
        let result = self.chomper.chomp(|ch| { ! ch.is_whitespace() });
        println!("Here's the whitespace chomp result: {}", result);
        if result.is_none() {fail!("You are not supposed to call get_whitespace unless you know you have some. But no whitespace was found.")}

        Token::make(result.unwrap().value, Whitespace, result.unwrap().startIndex, result.unwrap().endIndex) // todo unwrap too much
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
