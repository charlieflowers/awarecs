use chomp::*;

use collections::string::String;

// mod chomp; // If some other crate tries to use lex, then this won't work! That crate will have to say "mod chomp;" and "mod lex;"

macro_rules! crf {
    ($e:expr) => {
        println!("{} is {}", stringify!($e), $e);
    };
    ($e:expr BACKWARDS) => {
        println!("{} is what you get from {}", $e, stringify!($e));
    };
}

// pub fn main() {
//     println!("Hi charlie");
//     crf!(2i + 2);
//     crf!({
//         let y = 42i;
//         println!("The meaning of life is {}.", y);
//         y
//     });

//     crf!(2i+2 BACKWARDS);

// }

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
    Word,
    NewlineAndIndent,
    OpenQuote,
    StringFragment,
    OpenInterpolation,
    InterpolatedCode,
    CloseInterpolation,
    CloseQuote,
}

impl TokenTag {
    pub fn at<T>(&self, to_span: T) -> Token where T: ToSpan {
        Token::make(*self, *to_span.to_span())
    }

    pub fn assert_at<T>(&self, maybe_to_span: Option<T>) -> Token where T: ToSpan {
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

    pub fn text<'t, TSource>(&self, code: &'t TSource) -> String where TSource : SourceCodeProvider {
        format!("[{} {}]", self.tag, get_region(code, *self).to_string())
    }
}

pub trait SourceCodeProvider {
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

fn get_region<'x, TSource, TSpan>(source: &'x TSource, span: TSpan) -> &'x str where TSource: SourceCodeProvider, TSpan: ToSpan {
    let span = span.to_span();
    source.get_source_code().slice(span.start_pos.index, span.end_pos.index)
}

pub trait FullSource {
    fn get_slice<'x, TSpan, TSource>(&'x self, span: &TSpan) -> &'x str where TSpan: ToSpan, TSource: SourceCodeProvider;
}

impl<'coolness, T> FullSource for &'coolness T where T: SourceCodeProvider {
    fn get_slice<'x, TSpan, TSource>(&'x self, span: &TSpan) -> &'x str where TSpan: ToSpan, TSource: SourceCodeProvider {
        let span = span.to_span();
        self.get_source_code().slice(span.start_pos.index, span.end_pos.index)
    }
}

impl<'li> Lexer<'li> {

    pub fn new(code: &'li str) -> Lexer<'li> {
        Lexer {chomper: Chomper::new(code)}
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens : Vec<Token> = vec![];

        loop {
            if self.chomper.is_eof { break; }
            match self.chomper.peek() {
                None => break,
                Some(c) => {
                    let token = match c {
                        ch if Lexer::is_valid_first_char_of_word(ch) => self.get_word(),
                        '\n' => self.process_newline(),
                        '\"' => self.process_double_quote(&mut tokens),
                        ws if ws.is_whitespace() => self.get_whitespace(),
                        num if num.is_digit() => self.get_number(),
                        '+' | '-' => self.get_operator(),
                        '#' => self.get_comment(),
                        _ => {fail!("Charlie, you have not implemented ability to match char {} at index {}", c, self.chomper.index)}
                    };

                    println!("Got token!! {}", token);
                    println!("Chomper peek char is {}", self.chomper.peek());
                    println!("At this point, index is {}", self.chomper.index);

                    if token.is_some() {tokens.push(token.unwrap())};
                }
            }
        }

        tokens
    }

    pub fn process_double_quote(&mut self, token_list: &mut Vec<Token>) -> Option<Token> {

        inside_open_quote(self, token_list);
        return None;

        fn inside_open_quote(lexer: &mut Lexer, token_list: &mut Vec<Token>) {
            let open_quote_cr = lexer.chomper.expect("\"");
            token_list.push(OpenQuote.at(open_quote_cr));

            // todo charlie, clearly there is duplication here too! Come back to it down the road.

            loop {
                // let string_frag_cr = lexer.chomper.chomp_till_str(|s| s.starts_with("\"") || s.starts_with("#{"));
                let string_frag_cr = lexer.chomper.chomp_till_str_with_previous(|str, pc| (str.starts_with("\"") && pc != Some('\\')) || str.starts_with("#{"));
                if lexer.chomper.is_eof { fail!("Hit eof while parsing an interpolated string.")}
                if string_frag_cr.is_some() { token_list.push(StringFragment.at(string_frag_cr.unwrap())); }

                match lexer.chomper.peek().unwrap() {
                    '\"' => {
                        token_list.push(CloseQuote.at(lexer.chomper.expect("\"")));
                        return;
                    },
                    '#' => inside_open_interpolation(lexer, token_list),
                    unexpected_str => fail!("Got unexpected char: {}", unexpected_str),
                };
            }
        }

        fn inside_open_interpolation(lexer: &mut Lexer, token_list: &mut Vec<Token>) {
            let open_cr = lexer.chomper.expect("#{");
            token_list.push(OpenInterpolation.at(open_cr));

            loop {
                let code_frag_cr = lexer.chomper.chomp(|c| c == '}' || c == '\"');
                if lexer.chomper.is_eof { fail!("Hit eof while parsing some interpolation code inside an interpolated string.")}
                if code_frag_cr.is_some() { token_list.push(InterpolatedCode.at(code_frag_cr.unwrap())); }

                match lexer.chomper.peek().unwrap() {
                    '}' => {
                       token_list.push(CloseInterpolation.at(lexer.chomper.expect("}")));
                       return;
                     },
                    '\"' => inside_open_quote(lexer, token_list),
                    _ => unreachable!()
                };
            }
        }
    }

    pub fn get_word(&mut self) -> Option<Token> {
        fn fail(msg : String) {
            fail!("You called get_word, but the next char {}", msg);
        }
        match self.chomper.peek() {
            None => fail(String::from_str("is the end of file.")),
            Some(ch) => {
                if ! Lexer::is_valid_first_char_of_word(ch) {
                    fail(format!("is not a valid first char for a word. Char is {}", ch));
                }
            }
        };

        let first = self.chomper.chomp_count(1).unwrap();
        let rest = self.chomper.chomp(|c| { ! Lexer::is_valid_subsequent_char_of_word(c) });
        let span = (first + rest).span;

        Some(Word.at(span))
    }

    fn is_valid_first_char_of_word(ch: char) -> bool {
        match ch  {
            '$' | '_' => true,
            'A'..'Z' => true,
            'a'..'z' => true,
            _ => false
                // todo intentionally only allowing identifiers and words to start with "normal" ascii chars for now. Later, add support
                //   for higher ascii and unicode (/x7f - /uffff, as the reference coffeescript compiler does)
        }
    }

    fn is_valid_subsequent_char_of_word(ch: char) -> bool {
        match ch  {
            '$' | '_' => true,
            'a'..'z' => true,
            'A'..'Z' => true,
            '0'..'9' => true,
            _ => false
        }
    }

    pub fn get_whitespace(&mut self) -> Option<Token> { // todo, ONLY pub so you can test it, fix that later
        Some(Whitespace.assert_at(self.chomper.chomp(|ch| ! ch.is_whitespace() || ch == '\n')))
            // todo the wrong thing here is that the token Whitespace and the fn (|ch| ! ch.is_whitespace()) truly belong together. I'm repeating myself by saying that twice in this call
            // The answer is not necessarily the OO answer ... bundle it into the struct. Anything that associates the TokenTag with the scan fn makes sense, so think outside the oo box.
    }

    pub fn process_newline(&mut self) -> Option<Token> {
        Some(NewlineAndIndent.at(self.chomper.expect("\n") + self.chomper.chomp(|c| c == '\n' || ! c.is_whitespace())))
    }

    pub fn get_number(&mut self) -> Option<Token> {
        Some(Number.assert_at(self.chomper.chomp(|c| ! c.is_digit())))
    }

    pub fn get_operator(&mut self) -> Option<Token> {
        Some(Operator.assert_at(self.chomper.chomp(|c| c != '+' && c != '-')))
    }

    pub fn get_comment(&mut self) -> Option<Token> {
        // todo next line can be nicer
        if self.chomper.peek() != Some('#') { fail!("I thought I was parsing a comment, but it starts with this: {}", self.chomper.peek())}
        println!("seeing if we have herecomment");

        match self.chomper.text().slice_to(3) { // todo can probably pattern match more gracefully here
            "###" => self.get_here_comment(),
            _ => {
                println!("in get_comment, and decided it was NOT a herecomment.");
                println!("text is: {}", self.chomper.text());
                crf!(self.chomper.text());
                Some(Comment.assert_at(self.chomper.chomp(|c| c == '\n')))
            }
        }
    }

    pub fn get_here_comment(&mut self) -> Option<Token> {
        let delimiter = self.chomper.expect("###");
        if delimiter.hit_eof { return Some(Herecomment.at(delimiter)); }
        let mut cr = self.chomper.chomp_till_str(|str| str.starts_with("###")).unwrap();
        cr = delimiter + cr;

        if ! cr.hit_eof {
            cr = cr + self.chomper.expect("###");
        }

        Some(Herecomment.at(cr))
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
                                       start_pos: Position { index: 42, line_no: 42, col_no: 42 },
                                       end_pos: Position { index: 44, line_no: 44, col_no: 44 }
                                   },
                                   hit_eof: false});

        let token = Number.assert_at(cr);
        crf!(token);
        assert_eq!(token.tag, Number);
        assert_eq!(token.span.start_pos.index, 42);
        assert_eq!(token.span.end_pos.index, 44);
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
        assert_eq!(token.span.start_pos.index, 0);
        assert_eq!(token.span.end_pos.index, 3);
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
            let token_text = token.text(code);
            assert_eq!(token_text, expect.to_string());
            index = index + 1;
        }
    }

    #[test]
    fn should_be_able_to_lex_even_if_newline_is_last_thing_before_eof() {
        let code = "\n";
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        assert_tokens_match(&lexer, &tokens, vec!["[NewlineAndIndent \n]"]);
    }

    #[test]
    fn formula_with_no_spaces_should_succeed() {
        let code = "40+2\n";
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        dump_tokens_to_console(lexer, &tokens);
        assert_tokens_match(&lexer, &tokens, vec!["[Number 40]", "[Operator +]", "[Number 2]", "[NewlineAndIndent \n]"]);
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
        assert_tokens_match(&lexer, &tokens, vec!["[Number 40]", "[NewlineAndIndent \n]",
                                               "[Comment # This is a comment]",
                                               "[NewlineAndIndent \n]", "[Number 2]",
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
        assert_tokens_match(&lexer, &tokens, vec!["[NewlineAndIndent \n]", "[Number 40]", "[Whitespace  ]",
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
        assert_tokens_match(&lexer, &tokens, vec!["[NewlineAndIndent \n]", "[Number 40]", "[Whitespace  ]",
                                               "[Herecomment ### This whole thing right here is a\nherecomment that\nruns straight to EOF.]"]);
    }

    fn dump_tokens_to_console(code : Lexer, tokens: &Vec<Token> ) {
        let mut index :uint = 1;
        for t in tokens.iter() {
            println!("Token {} is {}", index, t.text(&code));
            index = index + 1;
        }
    }

    #[test]
    fn should_end_word_at_first_illegal_char() {
        let code = r#"someIden+tifier"#;
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        dump_tokens_to_console(lexer, &tokens);
        assert_tokens_match(&lexer, &tokens, vec!["[Word someIden]", "[Operator +]", "[Word tifier]"]);
    }

    #[test]
    fn should_be_able_to_lex_an_word_right_up_against_eof() {
        let code = r#"someWord"#;
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        assert_tokens_match(&lexer, &tokens, vec!["[Word someWord]"]);
    }

    #[test]
    fn should_lex_a_word_followed_by_a_number() {
        let code = "someWord 42";
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        assert_tokens_match(&lexer, &tokens, vec!["[Word someWord]", "[Whitespace  ]", "[Number 42]"]);
    }

    #[test]
    fn should_move_past_matched_text_when_calling_expect() {
        let code = r#"
123456789"#;
        let mut lexer = get_lexer(code);

        let cr = lexer.chomper.expect("\n");
        assert_eq!(Some('1'), lexer.chomper.peek());
        assert_eq!("123456789", lexer.chomper.text());
    }

    #[test]
    fn should_deal_with_newlines_correctly_for_my_pass_one() {
        let code = "40+2\n       \n   12";
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        assert_tokens_match(&lexer, &tokens, vec!["[Number 40]", "[Operator +]", "[Number 2]", "[NewlineAndIndent \n       ]", "[NewlineAndIndent \n   ]", "[Number 12]"]);
    }

    #[test]
    fn should_not_include_newline_in_whitespace() {
        let code = "     \n   ";
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        assert_tokens_match(&lexer, &tokens, vec!["[Whitespace      ]", "[NewlineAndIndent \n   ]"]);
    }

    #[test]
    fn should_give_newline_higher_precedence_than_whitespace() {
        let code = "\n";
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        assert_tokens_match(&lexer, &tokens, vec!["[NewlineAndIndent \n]"]);
    }

    #[test]
    fn should_lex_strings_with_interpolation_using_all_charlies_awesome_goodness() {
        let code = r#""The string is #{"The string".length} characters long""#;
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        assert_tokens_match(&lexer, &tokens, vec!["[OpenQuote \"]", "[StringFragment The string is ]", "[OpenInterpolation #{]", "[OpenQuote \"]",
                            "[StringFragment The string]", "[CloseQuote \"]", "[InterpolatedCode .length]", "[CloseInterpolation }]", "[StringFragment  characters long]",
                            "[CloseQuote \"]"]); // THIS WAS FUN!!
    }

    #[test]
    fn should_kick_the_same_ass_on_a_nested_interpolated_string_too() {
        let code = r#""The string is #{"The #{40 + 2}nd string".length} characters long""#;
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        assert_tokens_match(&lexer, &tokens, vec!["[OpenQuote \"]", "[StringFragment The string is ]", "[OpenInterpolation #{]",
            "[OpenQuote \"]", "[StringFragment The ]", "[OpenInterpolation #{]", "[InterpolatedCode 40 + 2]",
                                                  "[CloseInterpolation }]", "[StringFragment nd string]", "[CloseQuote \"]", "[InterpolatedCode .length]",
                            "[CloseInterpolation }]", "[StringFragment  characters long]",
                            "[CloseQuote \"]"]);
    }

    #[test]
    fn should_respect_escaped_quotes_in_non_interpolated_part_of_interpolated_strings() {
        let code = r#""The string \" is #{"The string".length} characters long""#;
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        assert_tokens_match(&lexer, &tokens, vec!["[OpenQuote \"]", "[StringFragment The string \\\" is ]", "[OpenInterpolation #{]", "[OpenQuote \"]",
                            "[StringFragment The string]", "[CloseQuote \"]", "[InterpolatedCode .length]", "[CloseInterpolation }]", "[StringFragment  characters long]",
                            "[CloseQuote \"]"]);
    }

    #[test]
    fn should_respect_escaped_quotes_in_interpolated_part_of_interpolated_strings() {
        let code = r#""The string is #{"The \"string".length} characters long""#;
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        assert_tokens_match(&lexer, &tokens, vec!["[OpenQuote \"]", "[StringFragment The string is ]", "[OpenInterpolation #{]", "[OpenQuote \"]",
                                                  "[StringFragment The \\\"string]", "[CloseQuote \"]", "[InterpolatedCode .length]",
                                                  "[CloseInterpolation }]", "[StringFragment  characters long]", "[CloseQuote \"]"]);
    }

    #[test]
    fn should_respect_brackets_as_literals_in_interpolated_strings_when_they_dont_close_an_interpolation() {
        let code = r#""This } string #{40 + 2} has 2 } that are merely literal brackets""#;
        let mut lexer = get_lexer(code);
        let tokens = lexer.lex();
        assert_tokens_match(&lexer, &tokens, vec!["[OpenQuote \"]", "[StringFragment This } string ]", "[OpenInterpolation #{]", "[InterpolatedCode 40 + 2]",
                                                  "[CloseInterpolation }]", "[StringFragment  has 2 } that are merely literal brackets]", "[CloseQuote \"]"]);
    }
}
