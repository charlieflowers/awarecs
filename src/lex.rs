#![feature(macro_rules)]
extern crate collections;

pub use chomp::{Chomper, Span, ToSpan, ChompResult, Position};
use collections::string::String;


mod chomp; // If some other crate tries to use lex, then this won't work! That crate will have to say "mod chomp;" and "mod lex;"

macro_rules! crf {
    ($e:expr) => {
        println!("{} is {}", stringify!($e), $e);
    };
    ($e:expr BACKWARDS) => {
        println!("{} is what you get from {}", $e, stringify!($e));
    };
}

pub fn main() {
    println!("Hi charlie");
    crf!(2i + 2);
    crf!({
        let y = 42i;
        println!("The meaning of life is {}.", y);
        y
    });

    crf!(2i+2 BACKWARDS);

}
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

    pub fn text<'t, TSource: SourceCodeProvider>(&self, code: &'t TSource) -> String {
        format!("[{} {}]", self.tag, get_region(code, *self).to_string())
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
        Whitespace.assert_at(self.chomper.chomp(|ch| ! ch.is_whitespace()))
            // todo the wrong thing here is that the token Whitespace and the fn (|ch| ! ch.is_whitespace()) truly belong together. I'm repeating myself by saying that twice in this call
            // The answer is not necessarily the OO answer ... bundle it into the struct. Anything that associates the TokenTag with the scan fn makes sense, so think outside the oo box.
    }

    pub fn get_number(&mut self) -> Token {
        Number.assert_at(self.chomper.chomp(|c| ! c.is_digit()))
    }

    pub fn get_operator(&mut self) -> Token {
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
                crf!(self.chomper.text());
                Comment.assert_at(self.chomper.chomp(|c| c == '\n'))
            }
        }
    }

    pub fn get_here_comment(&mut self) -> Token {
        let delimiter = self.chomper.expect("###");
        if delimiter.hitEof { return Herecomment.at(delimiter); }
        let mut cr = self.chomper.chomp_till_str(|str| str.starts_with("###")).unwrap();
        cr = delimiter + cr;

        if ! cr.hitEof {
            cr = cr + self.chomper.expect("###");
        }

        Herecomment.at(cr)
    }
}

#[cfg(test)]
mod test {
    use chomp::{Chomper, ChompResult, Span, Position};
    use super::{Token, Lexer, Number, Whitespace, FullSource, get_region};
    // not yet tested: SourceCodeProvider, TokenTag, Operator,

    #[test]
    fn option_chomp_result_that_is_some_should_be_convertable_to_token() {
        let cr = Some(ChompResult {
                                   span: Span {
                                       startPos: Position { index: 42, lineNo: 42, colNo: 42 },
                                       endPos: Position { index: 44, lineNo: 44, colNo: 44 }
                                   },
                                   hitEof: false});

        let token = Number.assert_at(cr);
        crf!(token);
        assert_eq!(token.tag, Number);
        assert_eq!(token.span.startPos.index, 42);
        assert_eq!(token.span.endPos.index, 44);
    }

    #[test]
    fn should_be_posssible_to_make_a_token_from_a_chomp_result() {
        let code = "foobar";
        let lexer = &get_lexer(code);
        let mut chomper = Chomper::new(code);

        let token = Whitespace.assert_at(chomper.chomp(|c| c == 'b')); // lying here. I'm calling it "Whitespace" cuz the TokenTag doesn't matter. It's not whitespace, and that's ok.

        println!("token is {}", token);
        assert_eq!(token.tag, Whitespace);
        assert_eq!(lexer.get_slice::<Token, Lexer>(&token), "foo");
        // assert_eq!(&(lexer.chomper).get_slice::<Token, &Chomper>(&token), "foo"); // And also why doesn't this work?? Lexer?
        // assert_eq!(&code.get_slice::<Token, &'static str>(&token), "foo"); // And I can't get anything that calls based on str to work either, even though the global function does just fine.
        // assert_eq!(lexer.get_slice(&token), "foo"); // This line doesn't work but it should. Why does the compiler need the type hint here???
        assert_eq!(get_region(lexer, token), "foo");
        assert_eq!(get_region(&code, token), "foo");
        assert_eq!(get_region(&lexer.chomper, token), "foo");
        assert_eq!(token.span.startPos.index, 0);
        assert_eq!(token.span.endPos.index, 3);
    }

    fn get_lexer<'code>(code: &'code str) -> Lexer<'code> {
        Lexer::new(code)
    }

    #[test]
    fn lex_should_handle_herecomment_starting_right_at_eof() {
        let code = "###";
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        assert_tokens_match(&lexer, &tokens, vec!["[Herecomment ###]"]);
    }

    #[test]
    fn hello_lex() {
        let code = r#"40 + 2
"#;
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        assert_tokens_match(&lexer, &tokens, vec!["[Number 40]", "[Whitespace  ]", "[Operator +]", "[Whitespace  ]", "[Number 2]"]);
    }

    fn assert_tokens_match(code: &Lexer, actualTokens: &Vec<Token>, expectations: Vec<&'static str>) {
        println!("Matching tokens: ");
        println!("   Expecting (length of {}): {}", expectations.len(), expectations);
        println!("   Actual (length of {}): {}", actualTokens.len(), actualTokens);

        let mut index = 0;
        let mut actualIter = actualTokens.iter();
        for expect in expectations.iter() {
            let token = actualIter.idx(index).unwrap();
            // let token_text = format!("[{} {}]", token.tag, get_region(code, *token).to_string());
            let token_text = token.text(code);
            assert_eq!(token_text, expect.to_string());
            index = index + 1;
        }
    }

    #[test]
    fn formula_with_no_spaces_should_succeed() {
        let code = r#"40+2
"#;
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        dump_tokens_to_console(lexer, &tokens);
        assert_tokens_match(&lexer, &tokens, vec!["[Number 40]", "[Operator +]", "[Number 2]"]);
    }

    #[test]
    #[should_fail]
    fn make_sure_assert_tokens_fails_when_it_should() {
        let code = "40+2";
        let mut lexer = get_lexer(code);
        let myTokens = lexer.lex();
        assert_tokens_match(&lexer, &myTokens, vec!["[WrongStuff +]"]);
    }

    #[test]
    fn should_handle_number_against_eof() {
        let code = r#"40+2"#;
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        assert_tokens_match(&lexer, &tokens, vec!["[Number 40]", "[Operator +]", "[Number 2]"]);
    }

    #[test]
    fn should_handle_comments_correctly() {
        let code = r#"40
# This is a comment
2 + 40"#;

        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        assert_tokens_match(&lexer, &tokens, vec!["[Number 40]", "[Whitespace \n]",
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
        let tokens = lexer.lex();
        assert_tokens_match(&lexer, &tokens, vec!["[Whitespace \n]", "[Number 40]", "[Whitespace  ]",
                                               "[Herecomment ### This whole thing right here is a\nherecomment that can span\nmany lines. A # in the middle is no problem. It won't end until\nthe proper ending delimiter is encountered. ###]"]);
    }

    #[test]
    fn should_handle_herecomments_that_hit_eof() {
        let code = r#"
40 ### This whole thing right here is a
herecomment that
runs straight to EOF."#;

        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        assert_tokens_match(&lexer, &tokens, vec!["[Whitespace \n]", "[Number 40]", "[Whitespace  ]",
                                               "[Herecomment ### This whole thing right here is a\nherecomment that\nruns straight to EOF.]"]);
    }

    fn dump_tokens_to_console(code : Lexer, tokens: &Vec<Token> ) {
        let mut index :uint = 1;
        for t in tokens.iter() {
            println!("Token {} is {}", index, t.text(&code));
            index = index + 1;
        }
    }
}