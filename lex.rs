#![feature(macro_rules)]
use chomp::{Chomper, ChompResult, Position, Span};

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
    pub span: Span,
    pub text: String
}

impl<'ti> Token<'ti> {
    pub fn make<'ti>(full_code: &'ti str, tag: TokenTag, span: Span) -> Token<'ti> {
        // todo get rid of the "text" field because it of course copies the whole source code & you worked so hard to avoid copies
        let my_slice = span.extract(full_code);

        Token {tag:tag, value: my_slice, span: span,
               text: ("[".to_string() + tag.to_string() + " " + my_slice.to_string() + "]").to_string()}
    }

    // Even this, the best idea i have so far, becomes a major pain in the ass! Because I need to take chomp, I need to take a mutable chomper. Because I need to return a
    //  Token, which requires a named lifetime, I must tie the lifetime of the returned Token to that of the incoming mutable chomper. Now, since I want to return these
    //  Tokens from my parser routines, those parser routines must return a token with the same lifetime as that of the chomper, which is a different lifetime from that
    //  of the chomper. Therefore, my parsing routines wind up needing their own named lifetime parameters. That forces lex() itself to need a named lifetime parameter.
    //  Basically, the picture is this: THIS SHIT IS PAINFUL!!!!! I think they need to make some improvements to the borrow checker and the trait resolution algorithm (as they
    //  already know and are working towards). Meanwhile, though, I must either find a much less painful way of working with Rust, or stop working with it and come back in
    //  say about 6 months.
    //
    // So, for now, I conclude this: having a struct with a reference in it simply becomes a major pain in the ass. It is nearly impossible to localize that pain to one
    //  small area. Instead, the pain spreads out like contagion. Therefore, try very hard to simply NEVER DO IT. That's not a good foundation for a language that wants to be
    //  expressive and joyful to use, so I hope they improve it. But it is still a workable approach that would be more pleasant than C not to mention god forsaken c++.
    pub fn make_helper<'a>(chomper: &'a mut Chomper, tag: TokenTag, charPredicate: |char| -> bool ) -> Token<'a> {
        Token::make(chomper.code, tag, chomper.chomp(charPredicate).expect(
            format!("You were expecting to see {}, but got None.", tag).as_slice()).span)
    }
}

impl<'li> Lexer<'li> {

    // Even these "static" methods don't work out. The lexer passed in needs a lifetime. And that lifetime somehow confuses the lifetime of the returned token!
    // fn s_make_token<'lim>(lexer: &'lim Lexer, cr: &ChompResult, tag: TokenTag) -> Token<'lim> {
    //     Token::make(lexer.chomper.code.slice(cr.span.startPos.index, cr.span.endPos.index), tag, cr.span)
    // }

    // fn s_make_token_opt<'limo>(lexer: &'limo Lexer, ocr: &Option<ChompResult>, tag: TokenTag) -> Token<'limo> {
    //     match *ocr {
    //         None => fail!("You tried to make a {} token, but you're at EOF.", tag),
    //         Some(ref cr) => Lexer::s_make_token(lexer, cr, tag)
    //     }
    // }

    // make_token and make_token_opt as impl methods on Lexer don't work out because of the overly strict borrow checker (issue # 6268)
    // fn make_token(&self, cr: &ChompResult, tag: TokenTag) -> Token<'li> {
    //     Token::make(self.chomper.code.slice(cr.span.startPos.index, cr.span.endPos.index), tag, cr.span)
    // }

    // fn make_token_opt(&self, ocr: &Option<ChompResult>, tag: TokenTag) -> Token<'li> {
    //     match *ocr {
    //         None => fail!("You tried to make a {} token, but you're at EOF.", tag),
    //         Some(ref cr) => self.make_token(cr, tag)
    //     }
    // }

    pub fn new(code: &'li str) -> Lexer<'li> {
        Lexer {chomper: Chomper::new(code)}
    }

    pub fn lex<'x>(&'x mut self) -> Vec<Token<'x>> {
        let mut tokens : Vec<Token> = vec![];

        loop {
            if self.chomper.isEof { break; }
            match self.chomper.peek() {
                None => break,
                Some(c) => {
                    let token = match c {
                        ch if Lexer::is_valid_first_char_of_identifier_or_keyword(ch) => self.get_identifier_or_keyword(),
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
        fn fail(msg : String) {
            fail!("You called get_identifier_or_keyword, but the next char {}", msg);
        }
        match self.chomper.peek() {
            None => fail(String::from_str("is the end of file.")),
            Some(ch) => {
                if ! Lexer::is_valid_first_char_of_identifier_or_keyword(ch) {
                    fail(format!("is not a valid first char for a word. Char is {}", ch));
                }
            }
        };

        let first = self.chomper.chomp_count(1).unwrap();
        let rest = self.chomper.chomp(|c| Lexer::is_valid_subsequent_char_of_identifier_or_keyword(c));
        let span = (first + rest).span;

        Token::make(self.chomper.code, Identifier, span)
    }

    fn is_valid_first_char_of_identifier_or_keyword(ch: char) -> bool {
        match(ch) {
            '$' | '_' => true,
            'A'..'Z' => true,
            'a'..'z' => true,
            _ => false
                // todo intentionally only allowing identifiers and words to start with "normal" ascii chars for now. Later, add support
                //   for higher ascii and unicode (/x7f - /uffff, as the reference coffeescript compiler does)
        }
    }

    fn is_valid_subsequent_char_of_identifier_or_keyword(ch: char) -> bool {
        match(ch) {
            '$' | '_' => true,
            'a'..'z' => true,
            'A'..'Z' => true,
            '0'..'9' => true,
            _ => false
        }
    }

    pub fn get_whitespace<'x>(&'x mut self) -> Token<'x> { // todo, ONLY pub so you can test it, fix that later
        // match self.chomper.chomp(|ch| ! ch.is_whitespace()) {
        //     None => fail!("You called get_whitespace, but no whitespace was found."),
        //     Some(ref cr) => self.make_token(cr, Whitespace)
        // }

        // self.make_token_opt(&self.chomper.chomp(|ch| ! ch.is_whitespace()), Whitespace)
        Token::make_helper(&self.chomper, Whitespace, |ch| ch.is_whitespace())
        // Token::make(self.chomper.code, Whitespace, self.chomper.chomp(|ch| ! ch.is_whitespace()).expect("You were expecting Whitespace, but got None.").span)
    }

    pub fn get_number<'x>(&'x mut self) -> Token<'x> {
        // self.make_token_opt(&self.chomper.chomp(|c| ! c.is_digit()), Number)
        Token::make_helper(&self.chomper, Number, |c| ! c.is_digit())

    }

    pub fn get_operator<'x>(&'x mut self) -> Token<'x> {
        // self.make_token_opt(&self.chomper.chomp(|c| c != '+' && c != '-'), Operator)
        Token::make_helper(&self.chomper, Operator, |c| c != '+' && c != '-')
    }

    pub fn get_comment<'x>(&'x mut self) -> Token<'x> {
        // todo next line can be nicer
        if self.chomper.peek() != Some('#') { fail!("I thought I was parsing a comment, but it starts with this: {}", self.chomper.peek())}
        println!("seeing if we have herecomment");

        match self.chomper.text().slice_to(3) { // todo can probably pattern match more gracefully here
            "###" => self.get_here_comment(),
            _ => {
                println!("in get_comment, and decided it was NOT a herecomment.");
                println!("text is: {}", self.chomper.text());
                // self.make_token_opt(&self.chomper.chomp(|c| c == '\n'), Comment)
                Token::make_helper(&self.chomper, Comment, |c| c == '\n')
            }
        }
    }

    pub fn get_here_comment(&mut self) -> Token<'li> {
        let delimiter = self.chomper.expect("###");
        if delimiter.hitEof {return Token::make(self.chomper.code, Herecomment, delimiter.span)};
        let mut cr = self.chomper.chomp_till_str(|str| str.starts_with("###")).unwrap();
        cr = delimiter + cr;

        if cr.hitEof {
            cr = cr + self.chomper.expect("###");
        }

        // let endIndex = match cr {
        //     ChompResult { hitEof: true, ..} => cr.endIndex,
        //     _ => {
        //         // todo I don't like that this appears to be an assignment, but it is actually doing something more
        //         self.chomper.expect("###");
        //         cr.endIndex + 3
        //     }
        // };

        // Token::make(self.chomper.code.slice(cr.startIndex - 3, endIndex), Herecomment, cr.startIndex - 3, endIndex)
        // self.make_token(&cr, Herecomment)
        Token::make(self.chomper.code, Herecomment, cr.span)
    }
}

#[cfg(test)]
mod test {
    use chomp::{Chomper, ChompResult, Span, Position};
    use super::{Token, Lexer, Number, Operator, Whitespace, TokenTag};

    #[test]
    fn option_chomp_result_that_is_some_should_be_convertable_to_token() {
        let cr = Some(ChompResult {
                                   span: Span {
                                       startPos: Position { index: 42, lineNo: 42, colNo: 42 },
                                       endPos: Position { index: 44, lineNo: 44, colNo: 44 }
                                   },
                                   hitEof: false});

        let full_code = "hi";
        let token = Token::make(full_code, Number, cr.unwrap().span);
        assert_eq!(token.tag, Number);
        assert_eq!(token.value, "hi");
        assert_eq!(token.span.startPos.index, 42);
        assert_eq!(token.span.endPos.index, 44);
        assert_eq!(token.text, "[Number hi]".to_string());
    }

    #[test]
    fn lex_should_be_able_to_make_a_token_from_a_chomp_result() {
        let code = "foobar";
        let mut lexer = get_lexer(code);
        let mut chomper = Chomper::new(code);
        // let token = lexer.make_token(&cr, Whitespace); // todo charlie, thinkabout why you wanted 1st parameter to be a reference
        let token = Token::make_helper(&chomper, Whitespace, |c| c == 'b');
        println!("token is {}", token);
        assert_eq!(token.tag, Whitespace);
        assert_eq!(token.value, "foo");
        assert_eq!(token.span.startPos.index, 0);
        assert_eq!(token.span.endPos.index, 3);
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

    fn make_unpositioned_token<'lt>(text: &'lt str, tag: TokenTag) -> Token<'lt> {
        Token::make(text, tag, Span {
            startPos: {Position { index: 0, lineNo: 0, colNo: 0}},
            endPos: {Position { index: 0, lineNo: 0, colNo: 0}}
        })
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
            make_unpositioned_token("40", Number),
            make_unpositioned_token("+", Operator),
            make_unpositioned_token("2", Number)];

        assert_tokens_match(&myTokens, vec!["[Number 40]", "[Operator +]", "[Number 2]"]);
    }

    #[test]
    #[should_fail]
    fn make_sure_assert_tokens_fails_when_it_should() {
        let code = "40+2";
        let myTokens = vec![
            make_unpositioned_token(code, Number),
            make_unpositioned_token(code, Operator),
            make_unpositioned_token(code, Number)];

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
