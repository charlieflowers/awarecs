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

trait ConvertableToToken<'traitLt> {
    fn to_token(&self, tag: TokenTag) -> Token<'traitLt>;
}

impl<'ctti> ConvertableToToken<'ctti> for ChompResult<'ctti> {
   fn to_token(&self, tag: TokenTag) -> Token<'ctti> {
       Token::make(self.value, tag, self.startIndex, self.endIndex)
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

    pub fn get_whitespace(&mut self) -> Token<'li> { // todo, ONLY pub so you can test it, fix that later
        match self.chomper.chomp(|ch| ! ch.is_whitespace()) {
            None => fail!("You called get_whitespace, but no whitespace was found."),
            Some(cr) => Token::make(cr.value, Whitespace, cr.startIndex, cr.endIndex) // you could print it here if you need to
        }
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

        // let mut endIndex = cr.endIndex;
        // if ! self.chomper.isEof {
        //     self.chomper.expect("###");
        //     endIndex = cr.endIndex + 3;
        // }
        Token::make(self.chomper.code.slice(cr.startIndex - 3, endIndex), Herecomment, cr.startIndex - 3, endIndex)
    }
}

#[cfg(test)]
mod test {
    use chomp::{Chomper};
    use super::{Token, Lexer, Number, Operator, Whitespace, ConvertableToToken};

    #[test]
    fn chomp_result_should_be_convertable_to_token() {
        let code = "foobar";
        let mut chomper = Chomper::new(code);
        let cr = chomper.chomp(|c| c == 'b').unwrap();
        let token = cr.to_token(Whitespace);
        println!("token is {}", token);
        // assert_eq!(token.tag, Whitespace); // todo why does this not compile?
        assert_eq!(token.value, "foo");
        assert_eq!(token.startIndex, 0);
        assert_eq!(token.endIndex, 3);
        // assert_eq!(token.text, "[Whitespace foo]"); // todo why does this not compile?
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
