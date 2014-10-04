pub use std::str::{Chars};
pub use std::iter::{Enumerate};

#[deriving(Show)]
#[deriving(PartialEq)]
pub struct ChompResult {
    pub hit_eof: bool,
    pub span: Span
}

// Fascinating what you get into when dealing with Option, and smacks very much of haskell monads. It drove me to
//   operator overloading, even though I don't really care about the '+' syntax. And also, had to do pcwalton's workaround
//   (entitled ""so what if I *want* overloading"") because each type can have each trait implemented only once.
//   Is the presence of Option enough to drive one to needing "overloading" and to these lengths? It sure seems appropriate in
//   this case, because without it i'd be doing a ton of monkey coding.

trait ICanBeTheRhsOfAddToChompResult {
    fn add_to_chomp_result(&self, lhs: &ChompResult) -> ChompResult;
}

impl<R: ICanBeTheRhsOfAddToChompResult> Add<R, ChompResult> for ChompResult {
    fn add(&self, rhs: &R) -> ChompResult {
        rhs.add_to_chomp_result(self)
    }
}

impl ICanBeTheRhsOfAddToChompResult for ChompResult {
    fn add_to_chomp_result(&self, lhs: &ChompResult) -> ChompResult {
        if self.span.start_pos.index != lhs.span.end_pos.index {
            fail!("The second ChompResult does not start immediately after the first one. First ChompResult: {}. Second ChompResult: {}", self, lhs);
        }

        ChompResult { span: Span { start_pos: lhs.span.start_pos, end_pos: self.span.end_pos }, hit_eof: self.hit_eof }
    }
}

impl ICanBeTheRhsOfAddToChompResult for Option<ChompResult> {
    fn add_to_chomp_result(&self, lhs: &ChompResult) -> ChompResult {
        match *self  {
            None => *lhs,
            Some(cr) => lhs + cr
        }
    }
}

#[deriving(Show)]
#[deriving(PartialEq)]
pub struct Position {
    pub index: uint,
    pub line_no: uint,
    pub col_no: uint
}

#[deriving(Show)]
#[deriving(PartialEq)]
pub struct Span {
    pub start_pos: Position,
    pub end_pos: Position
}

pub trait ToSpan {
    fn to_span(&self) -> &Span;
}

impl ToSpan for Span {
    fn to_span(&self) -> &Span {
        self
    }
}

impl ToSpan for ChompResult {
    fn to_span(&self) -> &Span {
        &self.span
    }
}

pub struct Chomper<'chomper> {
    pub code: &'chomper str,
    pub index: uint,
    char_iterator: Enumerate<Chars<'chomper>>,
    pub is_eof: bool,
    pub line_no: uint,
    pub col_no: uint,
}

impl<'ci> Chomper<'ci> {
    pub fn new(code: &'ci str) -> Chomper<'ci> {
        // don't forget, line numbers start at 1!!!!
        Chomper{code: code, index: 0, char_iterator: code.chars().enumerate(), is_eof: false, line_no: 1, col_no: 0}
    }

    pub fn position(&self) -> Position {
        Position { index: self.index, line_no: self.line_no, col_no: self.col_no }
    }

    fn assert_not_eof(&self) {
        if self.is_eof {fail!("Chomper is at EOF."); }
    }

    pub fn peek(&self) -> Option<char> {
        let target = self.index;
        if target >= self.code.len() {return None};
        Some(self.code.char_at(target))
    }

    pub fn text(&self) -> &'ci str {
        self.code.slice_from(self.index)
    }

    pub fn next(&mut self) -> Option<(uint, char)> {
        self.assert_not_eof();
        let result = self.char_iterator.next();
        self.index = self.index + 1;

        match result {
            None => {
                self.is_eof = true;
            },
            Some((_, '\n')) => {
                self.line_no = self.line_no + 1;
                self.col_no = 0;
            },
            _ => self.col_no = self.col_no + 1
        };

        return result;
    }

    pub fn expect(&mut self, expectation: &str) -> ChompResult {
        if ! self.text().starts_with(expectation) {
            fail!("At index {}, expected {} but got \r\n {}.", self.index, expectation, self.text())
        }

        self.chomp_count(expectation.len()).unwrap()
    }

    pub fn chomp_count(&mut self, count: uint) -> Option<ChompResult> {
        let mut chomped = -1;

        self.chomp(|_| {
            chomped = chomped + 1;
            chomped == count
        })
    }

    pub fn chomp_till_str(&mut self, quit: |&str| -> bool) -> Option<ChompResult> {
        self.chomp_internal(|_| false, quit)
    }

    pub fn chomp(&mut self, quit: |char| -> bool) -> Option<ChompResult> {
        self.chomp_internal(quit, |_| false)
    }

    fn chomp_internal(&mut self, char_quit: |char| -> bool, str_quit: |&str| -> bool) -> Option<ChompResult> {
        // What if chomper did not blow up on eof, but merely kept returning None? Of course, his flag will still say hitEof=true.
        // self.assert_not_eof();
        if self.is_eof  { return None; }

        let mut start_position: Option<Position> = None;
        let mut end_position: Option<Position> = None;

        println!("starting a chomp at text: {}", self.text());
        println!("index is: {}", self.index);
        println!("is_eof is {}", self.is_eof);
        println!("last valid index of code is {}", self.code.len() - 1);
        // todo I KNOW this can be simplified and cleaned up
        loop {
            let should_quit = match self.peek() {
                None => {
                    // This means, there IS no next character. EOF.
                    end_position = Some(self.position());
                    // Still need to call next(), to fully put chomper into EOF state.
                    self.next();
                    true
                },
                Some(ch) => {
                    if char_quit(ch) || str_quit(self.text()) {
                        end_position = Some(self.position());
                        true
                    } else {
                        println!("Not time to quit yet!");
                        if start_position == None {
                            println!("setting start index for chomp at {}", self.index);
                            start_position = Some(self.position());
                        }
                        self.next();
                        false
                    }
                }
            };

            if should_quit {
                println!("Just about to create ChompResult");
                println!("start_position is: {}", start_position);
                println!("end_position is: {}", end_position);

                if start_position == None {return None;}
                let cr = Some(ChompResult { span: Span { start_pos: start_position.unwrap(), end_pos: end_position.unwrap() },
                                            hit_eof: self.is_eof });

                println!("Full chomp result is: {}", cr);
                return cr;
            }
        }
    }

    pub fn value(&self, chomp_result: ChompResult) -> &'ci str {
        self.code.slice(chomp_result.span.start_pos.index, chomp_result.span.end_pos.index)
    }
}

#[cfg(test)]
mod test{
    use super::{Chomper, ChompResult};

    #[test]
    fn it_should_track_line_and_col_numbers() {
        let code = r#"This is
some text that starts at
line zero but then crosses many lines. I will
chomp it until 42, which is the first digit."#;

        let mut chomper = Chomper::new(code);
        let cr = chomper.chomp(|c| c.is_digit()).unwrap();
        assert_eq!(cr.span.start_pos.line_no, 1);
        assert_eq!(cr.span.start_pos.col_no, 0);

        assert_eq!(cr.span.end_pos.line_no, 4);
        assert_eq!(cr.span.end_pos.col_no, 15);
    }

    #[test]
    fn should_be_able_to_instantiate_chomper() {
        let code = "40 + 2";
        Chomper::new(code);
    }

    #[test]
    fn chomp_should_work_correctly_when_not_hitting_eof() {
        let code = "40 + 2";
        let mut chomper = Chomper::new(code);

        let result = chomper.chomp(|ch| { ! ch.is_digit() }).unwrap();

        assert_eq!(chomper.value(result), "40");
    }

    #[test]
    fn chomp_should_work_correctly_when_hitting_eof() {
        let code = "40";
        let mut chomper = Chomper::new(code);

        let result = chomper.chomp(|ch| {
            println!("Seeing if {} is a digit.", ch);
            ! ch.is_digit()
        }).unwrap();

        println!("result is: {}", result);

        assert_eq!(chomper.value(result), "40");
    }

    #[test]
    fn chomp_should_succeed_at_2_tokens_in_a_row() {
        let code = "40+2";
        let mut chomper = Chomper::new(code);

        let one = chomper.chomp(|c| ! c.is_digit()).unwrap();
        assert_eq!(chomper.value(one), "40");

        let two = chomper.chomp(|c| c != '+').unwrap();
        assert_eq!(chomper.value(two), "+");
    }

    #[test]
    fn chomp_should_return_none_if_youre_already_at_eof_when_you_call_it() {
        let code = "40";
        let mut chomper = Chomper::new(code);

        let chomper_borrow = &mut chomper;

        let result = chomper_borrow.chomp (|_| { false}).unwrap();
        assert_eq!(chomper_borrow.value(result), "40");

        let past_eof_cr = chomper_borrow.chomp(|_| { false });
        assert_eq!(past_eof_cr, None);
    }

    #[test]
    fn expect_should_work_for_happy_path() {
        let code = "foobar";
        let mut chomper = Chomper::new(code);
        chomper.expect("foobar");
    }

    #[test]
    fn expect_multiple_times_in_a_row_happy_path_should_work() {
        let code = "foobar";
        let mut chomper = Chomper::new(code);
        chomper.expect("foo");
        chomper.expect("bar");
    }

    #[test]
    #[should_fail]
    fn expect_should_work_for_failure_path() {
        let code = "foobar";
        let mut chomper = Chomper::new(code);
        chomper.expect("fooOOPSbar");
    }

    #[test]
    fn chomp_till_str_should_work_when_there_is_a_match() {
        let code = "This is some text";
        let mut chomper = Chomper::new(code);
        let cr = chomper.chomp_till_str(|str| str.starts_with("some")).unwrap();
        println!("the cr is {}", cr);
        assert_eq!(chomper.value(cr), "This is ");
        assert_eq!(cr.span.start_pos.index, 0);
        assert_eq!(cr.span.end_pos.index, 8);
        assert_eq!(chomper.is_eof, false);
    }

    #[test]
    fn chomp_till_str_should_work_when_there_is_no_match() {
        let code = "This is some text";
        let mut chomper = Chomper::new(code);
        let cr = chomper.chomp_till_str(|str| str.starts_with("XXXXXXX")).unwrap();
        println!("the cr is: {}", cr);
        assert_eq!(chomper.value(cr), "This is some text");
        assert_eq!(cr.span.start_pos.index, 0);
        assert_eq!(cr.span.end_pos.index, 17);
        assert_eq!(chomper.is_eof, true);
    }

    #[test]
    fn is_empty_should_be_true_if_you_quit_chomping_immediately() {
        let code = "foobar";
        let mut chomper = Chomper::new(code);
        let cr = chomper.chomp(|c| c == 'f');
        println!("cr is {}", cr);
        assert!(cr.is_none());
    }

    #[test]
    fn is_empty_should_be_false_if_you_even_one_char_is_chomped() {
        let code = "f";
        let mut chomper = Chomper::new(code);
        let cr = chomper.chomp(|_| false).unwrap();
        println!("cr is {}", cr);
    }

    #[test]
    fn adding_two_chomp_results_should_work_in_happy_path() {
        let code = "foobar";
        let mut chomper = Chomper::new(code);
        let one = chomper.expect("foo");
        let two = chomper.expect("bar");
        let combined = one + two;
        println!("add result = {}", combined);
        assert_eq!(chomper.value(combined), "foobar");
        assert_eq!(combined.span.start_pos.index, 0);
        assert_eq!(combined.span.end_pos.index, 6);
        assert_eq!(chomper.is_eof, true);
    }

    #[test]
    fn adding_some_to_chomp_result_should_work_in_happy_path() {
        let code = "foobar";
        let mut chomper = Chomper::new(code);
        let one = chomper.expect("foo");
        let two = Some(chomper.expect ("bar"));
        let combined = one + two;
        println!("add result = {}", combined);
        assert_eq!(chomper.value(combined), "foobar");
        assert_eq!(combined.span.start_pos.index, 0);
        assert_eq!(combined.span.end_pos.index, 6);
        assert_eq!(chomper.is_eof, true);
    }

    #[test]
    fn adding_none_to_chomp_result_should_work_in_happy_path() {
        let code = "foobar";
        let mut chomper = Chomper::new(code);
        let one = chomper.expect("foobar");
        let two: Option<ChompResult> = None;
        let combined = one + two;
        println!("add result = {}", combined);
        assert_eq!(chomper.value(combined), "foobar");
        assert_eq!(combined.span.start_pos.index, 0);
        assert_eq!(combined.span.end_pos.index, 6);
        assert_eq!(chomper.is_eof, true);
    }
}
