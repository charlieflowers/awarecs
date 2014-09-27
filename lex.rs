#![feature(macro_rules)]
pub use chomp::{Chomper, Span, ToSpan, ChompResult, Position};

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

impl TokenTag {
    pub fn at<T: ToSpan>(&self, to_span: T) -> Token {
        Token::make(*self, *to_span.to_span())
    }

    pub fn assert_at<T: ToSpan>(&self, maybe_to_span: Option<T>) -> Token {
        self.at(maybe_to_span.expect(format!("You were quite certain you would see the token {}, but you got None.", self).as_slice()))
    }
}

#[deriving(Show)]
pub struct Token {
    pub tag: TokenTag,
    pub span: Span,
}

impl Token {
    pub fn make(tag: TokenTag, span: Span) -> Token {
        Token {tag:tag, span: span}
    }
}

trait SourceCodeProvider {
    fn get_source_code<'x>(&'x self) -> &'x str;
}

impl ToSpan for Token {
    fn to_span(&self) -> &Span {
        &self.span
    }
}

impl<'s> SourceCodeProvider for &'s str {
    fn get_source_code<'x>(&'x self) -> &'x str {
        *self
    }
}

impl<'c> SourceCodeProvider for Chomper<'c> {
   fn get_source_code<'c>(&'c self) -> &'c str {
       self.code
   }
}

impl<'l> SourceCodeProvider for Lexer<'l> {
    fn get_source_code<'l>(&'l self) -> &'l str {
        self.chomper.code
    }
}

fn get_region<'x, TSource: SourceCodeProvider, TSpan: ToSpan>(source: &'x TSource, span: TSpan) -> &'x str {
    let span = span.to_span();
    source.get_source_code().slice(span.startPos.index, span.endPos.index)
}

trait FullSource {
    fn get_slice<'x, TSpan: ToSpan, TSource: SourceCodeProvider>(&'x self, span: &TSpan) -> &'x str;
}

impl<'coolness, T: SourceCodeProvider> FullSource for &'coolness T {
    fn get_slice<'x, TSpan: ToSpan, TSource: SourceCodeProvider>(&'x self, span: &TSpan) -> &'x str {
        let span = span.to_span();
        self.get_source_code().slice(span.startPos.index, span.endPos.index)
    }
}


impl<'li> Lexer<'li> {

    pub fn new(code: &'li str) -> Lexer<'li> {
        Lexer {chomper: Chomper::new(code)}
    }

    pub fn lex(&mut self) -> Vec<Token> {
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

    pub fn get_identifier_or_keyword(&mut self) -> Token {
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

        // Token::make(self.chomper.code, Identifier, span)
        Identifier.at(span)
    }

    fn is_valid_first_char_of_identifier_or_keyword(ch: char) -> bool {
        match ch  {
            '$' | '_' => true,
            'A'..'Z' => true,
            'a'..'z' => true,
            _ => false
                // todo intentionally only allowing identifiers and words to start with "normal" ascii chars for now. Later, add support
                //   for higher ascii and unicode (/x7f - /uffff, as the reference coffeescript compiler does)
        }
    }

    fn is_valid_subsequent_char_of_identifier_or_keyword(ch: char) -> bool {
        match ch  {
            '$' | '_' => true,
            'a'..'z' => true,
            'A'..'Z' => true,
            '0'..'9' => true,
            _ => false
        }
    }

    pub fn get_whitespace(&mut self) -> Token { // todo, ONLY pub so you can test it, fix that later
        // match self.chomper.chomp(|ch| ! ch.is_whitespace()) {
        //     None => fail!("You called get_whitespace, but no whitespace was found."),
        //     Some(ref cr) => self.make_token(cr, Whitespace)
        // }

        // self.make_token_opt(&self.chomper.chomp(|ch| ! ch.is_whitespace()), Whitespace)
        // Token::make_helper(&self.chomper, Whitespace, |ch| ch.is_whitespace())
        // Token::make(self.chomper.code, Whitespace, self.chomper.chomp(|ch| ! ch.is_whitespace()).expect("You were expecting Whitespace, but got None.").span)
        Whitespace.assert_at(self.chomper.chomp(|ch| ! ch.is_whitespace())) // todo the wrong thing here is that the token Whitespace and the fn (|ch| ! ch.is_whitespace()) truly belong together. I'm repeating myself by saying that twice in this call

    }

    pub fn get_number(&mut self) -> Token {
        // self.make_token_opt(&self.chomper.chomp(|c| ! c.is_digit()), Number)
        // Token::make_helper(&self.chomper, Number, |c| ! c.is_digit())
        Number.assert_at(self.chomper.chomp(|c| ! c.is_digit()))
    }

    pub fn get_operator(&mut self) -> Token {
        // self.make_token_opt(&self.chomper.chomp(|c| c != '+' && c != '-'), Operator)
        // Token::make_helper(&self.chomper, Operator, |c| c != '+' && c != '-')
        Operator.assert_at(self.chomper.chomp(|c| c != '+' && c != '-'))
    }

    pub fn get_comment(&mut self) -> Token {
        // todo next line can be nicer
        if self.chomper.peek() != Some('#') { fail!("I thought I was parsing a comment, but it starts with this: {}", self.chomper.peek())}
        println!("seeing if we have herecomment");

        match self.chomper.text().slice_to(3) { // todo can probably pattern match more gracefully here
            "###" => self.get_here_comment(),
            _ => {
                println!("in get_comment, and decided it was NOT a herecomment.");
                println!("text is: {}", self.chomper.text());
                // self.make_token_opt(&self.chomper.chomp(|c| c == '\n'), Comment)
                // Token::make_helper(&self.chomper, Comment, |c| c == '\n')
                Comment.assert_at(self.chomper.chomp(|c| c == '\n'))
            }
        }
    }

    pub fn get_here_comment(&mut self) -> Token {
        let delimiter = self.chomper.expect("###");
        // if delimiter.hitEof {return Token::make(self.chomper.code, Herecomment, delimiter.span)};
        if delimiter.hitEof { return Herecomment.at(delimiter); }
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
        // Token::make(self.chomper.code, Herecomment, cr.span)
        Herecomment.at(cr)
    }
}

#[cfg(test)]
mod test {
    use chomp::{Chomper, ChompResult, Span, Position};
    use super::{Token, Lexer, Number, Operator, Whitespace, TokenTag, SourceCodeProvider, FullSource, get_region};

    #[test]
    fn option_chomp_result_that_is_some_should_be_convertable_to_token() {
        let cr = Some(ChompResult {
                                   span: Span {
                                       startPos: Position { index: 42, lineNo: 42, colNo: 42 },
                                       endPos: Position { index: 44, lineNo: 44, colNo: 44 }
                                   },
                                   hitEof: false});

        let token = Number.assert_at(cr);
        assert_eq!(token.tag, Number);
        assert_eq!(token.span.startPos.index, 42);
        assert_eq!(token.span.endPos.index, 44);
    }

    #[test]
    fn should_be_posssible_to_make_a_token_from_a_chomp_result() {
        let code = "foobar";
        let mut lexer = &Lexer::new(code);
        let mut chomper = Chomper::new(code);
        // let token = lexer.make_token(&cr, Whitespace); // todo charlie, thinkabout why you wanted 1st parameter to be a reference
        // let token = Token::make_helper(&chomper, Whitespace, |c| c == 'b');

        let token = Whitespace.assert_at(chomper.chomp(|c| c == 'b')); // lying here. I'm calling it "Whitespace" cuz the TokenTag doesn't matter. It's not whitespace, and that's ok.

        println!("token is {}", token);
        assert_eq!(token.tag, Whitespace);
        assert_eq!(lexer.get_slice::<Token, Lexer>(&token), "foo");
        // assert_eq!(lexer.get_slice(&token), "foo"); // This line doesn't work but it should. Why does the compiler need the type hint here???
        assert_eq!(get_region(lexer, token), "foo");
        assert_eq!(token.span.startPos.index, 0);
        assert_eq!(token.span.endPos.index, 3);
    }

    fn get_lexer<'code>(code: &'code str) -> Lexer<'code> {
        Lexer::new(code)
    }

    // #[test]
    // fn lex_should_handle_herecomment_starting_right_at_eof() {
    //     let code = "###";
    //     let mut lexer = get_lexer(code);
    //     let tokens = lexer.lex();
    //     assert_tokens_match(&tokens, vec!["[Herecomment ###]"]);
    // }

//     #[test]
//     fn hello_lex() {
//         let code = r#"40 + 2
// "#;
//         let mut lexer = get_lexer(code);
//         let tokens = lexer.lex();
//         assert_tokens_match(&tokens, vec!["[Number 40]", "[Whitespace  ]", "[Operator +]", "[Whitespace  ]", "[Number 2]"]);
//     }

//     fn assert_tokens_match(actualTokens: &Vec<Token>, expectations: Vec<&'static str>) {
//         println!("Matching tokens: ");
//         println!("   Expecting (length of {}): {}", expectations.len(), expectations);
//         println!("   Actual (length of {}): {}", actualTokens.len(), actualTokens);

//         let mut index = 0;
//         let mut actualIter = actualTokens.iter();
//         for expect in expectations.iter() {
//             let token = actualIter.idx(index).unwrap();
//             assert_eq!(token.text, expect.to_string());
//             index = index + 1;
//         }
//     }

//     fn make_unpositioned_token(tag: TokenTag) -> Token {
//         Token::make(tag, Span {
//             startPos: {Position { index: 0, lineNo: 0, colNo: 0}},
//             endPos: {Position { index: 0, lineNo: 0, colNo: 0}}
//         })
//     }

//     #[test]
//     fn formula_with_no_spaces_should_succeed() {
//         let code = r#"40+2
// "#;
//         let mut lexer = get_lexer(code);
//         let tokens = lexer.lex();
//         dump_tokens_to_console(&tokens);
//         assert_tokens_match(&tokens, vec!["[Number 40]", "[Operator +]", "[Number 2]"]);
//     }

//     #[test]
//     fn make_sure_assert_tokens_itself_works() {
//         let myTokens = vec![
//             make_unpositioned_token(Number),
//             make_unpositioned_token(Operator),
//             make_unpositioned_token(Number)];

//         assert_tokens_match(&myTokens, vec!["[Number 40]", "[Operator +]", "[Number 2]"]);
//     }

//     #[test]
//     #[should_fail]
//     fn make_sure_assert_tokens_fails_when_it_should() {
//         let code = "40+2";
//         let myTokens = vec![
//             make_unpositioned_token(Number),
//             make_unpositioned_token(Operator),
//             make_unpositioned_token(Number)];

//         assert_tokens_match(&myTokens, vec!["[WrongStuff +]"]);
//     }

//     #[test]
//     fn should_handle_number_against_eof() {
//         let code = r#"40+2"#;
//         let mut lexer = get_lexer(code);
//         assert_tokens_match(&lexer.lex(), vec!["[Number 40]", "[Operator +]", "[Number 2]"]);
//     }

//     #[test]
//     fn should_handle_comments_correctly() {
//         let code = r#"40
// # This is a comment
// 2 + 40"#;

//         let mut lexer = get_lexer(code);
//         assert_tokens_match(&lexer.lex(), vec!["[Number 40]", "[Whitespace \n]",
//                                                "[Comment # This is a comment]",
//                                                "[Whitespace \n]", "[Number 2]",
//                                                "[Whitespace  ]", "[Operator +]", "[Whitespace  ]", "[Number 40]"]);
//     }

//     #[test]
//     fn should_handle_herecomments_correctly() {
//         let code = r#"
// 40 ### This whole thing right here is a
// herecomment that can span
// many lines. A # in the middle is no problem. It won't end until
// the proper ending delimiter is encountered. ###"#;

//         let mut lexer = get_lexer(code);
//         assert_tokens_match(&lexer.lex(), vec!["[Whitespace \n]", "[Number 40]", "[Whitespace  ]",
//                                                "[Herecomment ### This whole thing right here is a\nherecomment that can span\nmany lines. A # in the middle is no problem. It won't end until\nthe proper ending delimiter is encountered. ###]"]);
//     }

//     #[test]
//     fn should_handle_herecomments_that_hit_eof() {
//         let code = r#"
// 40 ### This whole thing right here is a
// herecomment that
// runs straight to EOF."#;

//         let mut lexer = get_lexer(code);
//         assert_tokens_match(&lexer.lex(), vec!["[Whitespace \n]", "[Number 40]", "[Whitespace  ]",
//                                                "[Herecomment ### This whole thing right here is a\nherecomment that\nruns straight to EOF.]"]);
//     }

    // fn dump_tokens_to_console(tokens: &Vec<Token> ) {
    //     let mut index :uint = 1;
    //     for t in tokens.iter() {
    //         println!("Token {} is {}", index, t.text);
    //         index = index + 1;
    //     }
    // }
}
