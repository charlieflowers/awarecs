#![feature(macro_rules)]
use chomp::{Chomper, ChompResult};

mod chomp; // If some other crate tries to use lex, then this won't work! That crate will have to say "mod chomp;" and "mod lex;"

// I think rust's module system needs some simplification. It is crazy that, even though my lex module depends on my chomp module, the lex
//  module CANNOT say, right here, "import the chomp mod". Rather, whatever the "crate root" is must import both modules.
//  For example see the test mod for lex.rs, which, at the top, says "mod chomp;" and "mod lex;". The crate root must call "mod" for all
//  necessary modules, and then the individual modules can get shorter names by "use"-ing them.

pub struct Lexer<'lexer> {
    chomper: Chomper<'lexer>,
}

#[deriving(Show)]
#[deriving(PartialEq)]
pub enum TokenTag {
    Number,
    Whitespace,
    Operator,
    Herecomment,
    Comment,
    Identifier,
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

trait ConvertableToToken<'traitLt> {
    fn to_token(&self, tag: TokenTag) -> Token<'traitLt>;
}

impl<'ctti> ConvertableToToken<'ctti> for ChompResult<'ctti> {
   fn to_token(&self, tag: TokenTag) -> Token<'ctti> {
       Token::make(self.value, tag, self.startIndex, self.endIndex)
   }
}

impl<'ctti> ConvertableToToken<'ctti> for Option<ChompResult<'ctti>> {
   fn to_token(&self, tag: TokenTag) -> Token<'ctti> {
       match *self {
           None => fail!("You are trying to make a {} token from a ChompResult of None!", tag),
           Some(cr) => cr.to_token(tag)
       }
   }
}

impl<'li> Lexer<'li> {
    pub fn new(code: &'li str) -> Lexer<'li> {
        Lexer {chomper: Chomper::new(code)}
    }

    pub fn lex(&mut self) -> Vec<Token<'li>> {
        let mut tokens : Vec<Token> = vec![];

        loop {
            if self.chomper.isEof { break; }
            match self.chomper.peek() {
                None => break,
                Some(c) => {
                    let token = match c {
                        ch if ch.is_valid_first_char_of_identifier_or_keyword() => self.get_identifier_or_keyword(),
                        ws if ws.is_whitespace() => self.get_whitespace(),
                        num if num.is_digit() => self.get_number(),
                        '+' | '-' => self.get_operator(),
                        '#' => self.get_comment(),
                        _ => {fail!("Charlie, you have not implemented ability to match char {} at index {}", c, self.chomper.index)}
                    };

                    println!("Got token!! {}", token);
                    println!("Chomper peek char is {}", self.chomper.peek());
                    println!("At this point, index is {}", self.chomper.index);

                    tokens.push(token);
                }
            }
        }

        tokens
    }

    pub fn get_identifier_or_keyword(&mut self) -> Token<'li> {
        if ! self.chomper.peek().is_valid_first_char_of_identifier_or_keyword() {
            fail!("You called get_identifier_or_keyword, but the next char is not a valid first char for a word. Char is: {}", self.chomper.peek());
        }

        let first = self.chomper.chomp_count(1);
        let rest = self.chomper.chomp(|c| is_valid_subsequent_char_of_identifier_or_keyword(c));

        first.combine(rest, self.code).to_token(Identifier)
    }

    fn is_valid_first_char_of_identifier_or_keyword(char ch) -> bool {
        match(ch) {
            '$' | '_' => true,
            'A'..'Z' => true,
            'a'..'z' => true,
            _ => false
                // todo intentionally only allowing identifiers and words to start with "normal" ascii chars for now. Later, add support
                //   for higher ascii and unicode (/x7f - /uffff, as the reference coffeescript compiler does)
        }
    }

    fn is_valid_subsequent_char_of_identifier_or_keyword(char ch) -> bool {
        match(ch) {
            '$' | '_' => true,
            'a'..'z' => true,
            'A'..'Z' => true,
            '0'..'9' => true,
            _ => false
        }
    }

    pub fn get_identifier_or_keyword(&mut self) -> Token<'li> {

    }

    pub fn get_whitespace(&mut self) -> Token<'li> { // todo, ONLY pub so you can test it, fix that later
        match self.chomper.chomp(|ch| ! ch.is_whitespace()) {
            None => fail!("You called get_whitespace, but no whitespace was found."),
            Some(cr) => cr.to_token(Whitespace)
        }
    }

    pub fn get_number(&mut self) -> Token<'li> {
        self.chomper.chomp(|c| ! c.is_digit()).to_token(Number)
    }

    pub fn get_operator(&mut self) -> Token<'li> {
        self.chomper.chomp(|c| c != '+' && c != '-').to_token(Operator)
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
                self.chomper.chomp(|c| c == '\n').to_token(Comment)
            }
        }
    }

    pub fn get_here_comment(&mut self) -> Token<'li> {
        let delimiter = self.chomper.expect("###");
        if delimiter.hitEof {return delimiter.to_token(Herecomment)};
        let cr = self.chomper.chomp_till_str(|str| str.starts_with("###")).unwrap();

        let endIndex = match cr {
            ChompResult { hitEof: true, ..} => cr.endIndex,
            _ => {
                // todo I don't like that this appears to be an assignment, but it is actually doing something more
                self.chomper.expect("###");
                cr.endIndex + 3
            }
        };

        Token::make(self.chomper.code.slice(cr.startIndex - 3, endIndex), Herecomment, cr.startIndex - 3, endIndex)
    }
}

#[cfg(test)]
mod test {
    use chomp::{Chomper, ChompResult};
    use super::{Token, Lexer, Number, Operator, Whitespace, ConvertableToToken};

    #[test]
    fn option_chomp_result_that_is_some_should_be_convertable_to_token() {
        let cr = Some(ChompResult {value: "hi", startIndex: 42, endIndex: 44, hitEof: false});
        let token = cr.to_token(Number);
        assert_eq!(token.tag, Number);
        assert_eq!(token.value, "hi");
        assert_eq!(token.startIndex, 42);
        assert_eq!(token.endIndex, 44);
        assert_eq!(token.text, "[Number hi]".to_string());
    }

    #[test]
    #[should_fail]
    fn option_chomp_result_that_is_none_should_fail_to_convert_to_token() {
        let cr : Option<ChompResult> = None;
        cr.to_token(Number);
    }

    #[test]
    fn chomp_result_should_be_convertable_to_token() {
        let code = "foobar";
        let mut chomper = Chomper::new(code);
        let cr = chomper.chomp(|c| c == 'b').unwrap();
        let token = cr.to_token(Whitespace);
        println!("token is {}", token);
        assert_eq!(token.tag, Whitespace);
        assert_eq!(token.value, "foo");
        assert_eq!(token.startIndex, 0);
        assert_eq!(token.endIndex, 3);
        assert_eq!(token.text, "[Whitespace foo]".to_string());
    }

    #[test]
    fn lex_should_handle_herecomment_starting_right_at_eof() {
        let code = "###";
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        assert_tokens_match(&tokens, vec!["[Herecomment ###]"]);
    }

    #[test]
    fn hello_lex() {
        let code = r#"40 + 2
"#;
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        assert_tokens_match(&tokens, vec!["[Number 40]", "[Whitespace  ]", "[Operator +]", "[Whitespace  ]", "[Number 2]"]);
    }

    fn get_lexer<'code>(code: &'code str) -> Lexer<'code> {
        Lexer::new(code)
    }

    fn assert_tokens_match(actualTokens: &Vec<Token>, expectations: Vec<&'static str>) {
        println!("Matching tokens: ");
        println!("   Expecting (length of {}): {}", expectations.len(), expectations);
        println!("   Actual (length of {}): {}", actualTokens.len(), actualTokens);

        let mut index = 0;
        let mut actualIter = actualTokens.iter();
        for expect in expectations.iter() {
            let token = actualIter.idx(index).unwrap();
            assert_eq!(token.text, expect.to_string());
            index = index + 1;
        }
    }

    #[test]
    fn formula_with_no_spaces_should_succeed() {
        let code = r#"40+2
"#;
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        dump_tokens_to_console(&tokens);
        assert_tokens_match(&tokens, vec!["[Number 40]", "[Operator +]", "[Number 2]"]);
    }

    #[test]
    fn make_sure_assert_tokens_itself_works() {
        let myTokens = vec![
            Token::make("40", Number, 0, 2),
            Token::make("+", Operator, 2, 3),
            Token::make("2", Number, 3, 4)];

        assert_tokens_match(&myTokens, vec!["[Number 40]", "[Operator +]", "[Number 2]"]);
    }

    #[test]
    #[should_fail]
    fn make_sure_assert_tokens_fails_when_it_should() {
        let code = "40+2";
        let myTokens = vec![
            Token::make(code, Number, 0, 2),
            Token::make(code, Operator, 2, 3),
            Token::make(code, Number, 3, 4)];

        assert_tokens_match(&myTokens, vec!["[WrongStuff +]"]);
    }

    #[test]
    fn should_handle_number_against_eof() {
        let code = r#"40+2"#;
        let mut lexer = get_lexer(code);
        assert_tokens_match(&lexer.lex(), vec!["[Number 40]", "[Operator +]", "[Number 2]"]);
    }

    #[test]
    fn should_handle_comments_correctly() {
        let code = r#"40
# This is a comment
2 + 40"#;

        let mut lexer = get_lexer(code);
        assert_tokens_match(&lexer.lex(), vec!["[Number 40]", "[Whitespace \n]",
                                               "[Comment # This is a comment]",
                                               "[Whitespace \n]", "[Number 2]",
                                               "[Whitespace  ]", "[Operator +]", "[Whitespace  ]", "[Number 40]"]);
    }

    #[test]
    fn should_handle_herecomments_correctly() {
        let code = r#"
40 ### This whole thing right here is a
herecomment that can span
many lines. A # in the middle is no problem. It won't end until
the proper ending delimiter is encountered. ###"#;

        let mut lexer = get_lexer(code);
        assert_tokens_match(&lexer.lex(), vec!["[Whitespace \n]", "[Number 40]", "[Whitespace  ]",
                                               "[Herecomment ### This whole thing right here is a\nherecomment that can span\nmany lines. A # in the middle is no problem. It won't end until\nthe proper ending delimiter is encountered. ###]"]);
    }

    #[test]
    fn should_handle_herecomments_that_hit_eof() {
        let code = r#"
40 ### This whole thing right here is a
herecomment that
runs straight to EOF."#;

        let mut lexer = get_lexer(code);
        assert_tokens_match(&lexer.lex(), vec!["[Whitespace \n]", "[Number 40]", "[Whitespace  ]",
                                               "[Herecomment ### This whole thing right here is a\nherecomment that\nruns straight to EOF.]"]);
    }

    fn dump_tokens_to_console(tokens: &Vec<Token> ) {
        let mut index :uint = 1;
        for t in tokens.iter() {
            println!("Token {} is {}", index, t.text);
            index = index + 1;
        }
    }
}
