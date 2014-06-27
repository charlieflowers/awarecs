pub struct Lexer {
    meaningOfLife: uint
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {meaningOfLife: 42}
    }

    pub fn lex(&self, code:&str) -> Vec<Token> {
        vec![Token::make (Number, "42".to_owned(), 4200)]
    }
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
pub struct Token {
    pub tag: TokenTag,
    value: ~str,
    length: uint,
    index: uint,
    pub text: ~str
}

impl Token {
    // fn new(tag: TokenTag, value: ~str, length: uint, index: uint) -> Token {
    //     Token {tag:tag, value:value, length:length, index:index}
    // }
    pub fn make(tag: TokenTag, value: &str, endingIndex: uint) -> Token {
        let fucking_len = value.len();
        let fucking_index = endingIndex - fucking_len;

        Token {tag:tag, value:value.to_owned(), length: fucking_len, index: fucking_index,
               text: tag.to_str().to_owned() + ": ".to_owned() + value.to_owned()}
    }
}
