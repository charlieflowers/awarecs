pub use std::str::{Chars};
pub use std::iter::{Enumerate};

#[deriving(Show)]
pub struct ChompResult {
    pub hitEof: bool,
    pub span: Span
}

// impl<'cri> ChompResult<'cri> {
//     pub fn combine(&mut self, target: ChompResult, code: &'cri str) -> ChompResult<'cri> {
//         if self.span.startPos.index >= target.span.startPos.index {
//             fail!("The second ChompResult does not start immediately after the first one.");
//         }

//         ChompResult { span: Span { startPos: self.span.startPos, endPos: target.span.endPos },
//                       hitEof: target.hitEof,
//                       value: code.slice(self.span.startPos.index, target.span.endPos.index) }
//     }
// }

// Fascinating what you get into when dealing with Option, and smacks very much of haskell monads. It drove me to
//   operator overloading, even though I don't really care about the '+' syntax. And also, had to do pcwalton's workaround
//   (entitled ""so what if I *want* overloading"") because each type can have each trait implemented only once.
//   Is the presence of Option enough to drive one to needing "overloading" and to these lengths? It sure seems appropriate in
//   this case, because without it i'd be doing a ton of monkey coding.

trait ICanBeTheRhsOfAddToChompResult { // I am having my own fun with these lifetime names, so butt out :)
    fn add_to_chomp_result(&self, lhs: &ChompResult) -> ChompResult;
}

// trait ICanBeTheRhsOfAddToChompResult<'ticbroacr> { // I am having my own fun with these lifetime names, so butt out :)
//     fn add_to_chomp_result(&self, lhs: &'ticbroacr ChompResult) -> ChompResult<'ticbroacr>;
// }

// impl<'zzzz, R: ICanBeTheRhsOfAddToChompResult> Add<R, ChompResult<'zzzz>> for ChompResult<'zzzz> {
//     fn add(&'zzzz self, rhs: &'zzzz R) -> ChompResult<'zzzz> {
//         rhs.add_to_chomp_result(self)
//     }
// }

impl<R: ICanBeTheRhsOfAddToChompResult> Add<R, ChompResult> for ChompResult {
    fn add(&self, rhs: &R) -> ChompResult {
        rhs.add_to_chomp_result(self)
    }
}

// impl<'iicbroacr> ICanBeTheRhsOfAddToChompResult<'iicbroacr> for ChompResult<'iicbroacr> {
//     fn add_to_chomp_result(&self, lhs: &ChompResult) -> ChompResult<'iicbroacr> {
//         if(lhs.span.startPos.index != self.span.startPos.index - 1) {
//             fail!("The second ChompResult does not start immediately after the first one.");
//         }

//         ChompResult { span: Span { startPos: lhs.span.startPos, endPos: self.span.endPos },
//                       hitEof: self.hitEof, fullCode: self.fullCode,
//                       value: self.fullCode.slice(lhs.span.startPos.index, self.span.endPos.index) }
//     }
// }

// impl<'iicbroacr> ICanBeTheRhsOfAddToChompResult<'iicbroacr> for ChompResult<'iicbroacr> {
//     fn add_to_chomp_result<'iicbroacr>(&'iicbroacr self, lhs: &'iicbroacr ChompResult) -> ChompResult<'iicbroacr> {
//         if(lhs.span.startPos.index != self.span.startPos.index - 1) {
//             fail!("The second ChompResult does not start immediately after the first one.");
//         }

//         ChompResult { span: Span { startPos: lhs.span.startPos, endPos: self.span.endPos },
//                       hitEof: self.hitEof, fullCode: self.fullCode,
//                       value: self.fullCode.slice(lhs.span.startPos.index, self.span.endPos.index) }
//     }
// }

impl ICanBeTheRhsOfAddToChompResult for ChompResult {
    fn add_to_chomp_result(&self, lhs: &ChompResult) -> ChompResult {
        if self.span.startPos.index != lhs.span.endPos.index {
            fail!("The second ChompResult does not start immediately after the first one. First ChompResult: {}. Second ChompResult: {}", self, lhs);
        }

        ChompResult { span: Span { startPos: lhs.span.startPos, endPos: self.span.endPos }, hitEof: self.hitEof }
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

// impl<'iicbroacrfo> ICanBeTheRhsOfAddToChompResult<'iicbroacrfo> for Option<ChompResult<'iicbroacrfo>> {
//     fn add_to_chomp_result(&self, lhs: &ChompResult<'iicbroacrfo>) -> ChompResult<'iicbroacrfo> {
//         match(*self) {
//             None => *lhs,
//             Some(cr) => lhs + cr
//         }
//     }
// }

// impl<'acri> Add<ChompResult<'acri>, ChompResult<'acri>> for ChompResult<'acri> {
//     fn add(&self, rhs: &ChompResult<'acri>) -> ChompResult<'acri> {
//         if self.span.startPos.index >= rhs.span.startPos.index {
//             fail!("The second ChompResult does not start immediately after the first one.");
//         }

//         ChompResult { fullCode: self.fullCode, hitEof: rhs.hitEof,
//                       span: Span { startPos: self.span.startPos, endPos: rhs.span.endPos },
//                       value: self.fullCode.slice(self.span.startPos.index, rhs.span.endPos.index) }
//     }
// }

// impl<'acri> Add<Option<ChompResult<'acri>>, ChompResult<'acri>> for ChompResult<'acri> {
//     fn add(&self, rhs: &Option<ChompResult<'acri>>) -> ChompResult<'acri> {
//         match(rhs) {
//             None => *self,
//             Some(target) => *self + *target
//         }
//     }
// }

#[deriving(Show)]
#[deriving(PartialEq)]
pub struct Position {
    pub index: uint,
    pub lineNo: uint,
    pub colNo: uint
}

#[deriving(Show)]
#[deriving(PartialEq)]
pub struct Span {
    pub startPos: Position,
    pub endPos: Position
}

pub struct Chomper<'chomper> {
    pub code: &'chomper str,
    pub index: uint,
    char_iterator: Enumerate<Chars<'chomper>>,
    pub isEof: bool,
    pub lineNo: uint,
    pub colNo: uint,
}

impl<'ci> Chomper<'ci> {
    pub fn new(code: &'ci str) -> Chomper<'ci> {
        // don't forget, line numbers start at 1!!!!
        Chomper{code: code, index: 0, char_iterator: code.chars().enumerate(), isEof: false, lineNo: 1, colNo: 0}
    }

    pub fn position(&self) -> Position {
        Position { index: self.index, lineNo: self.lineNo, colNo: self.colNo }
    }

    fn assert_not_eof(&self) {
        if self.isEof {fail!("Chomper is at EOF."); }
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
                self.isEof = true;
            },
            Some((_, '\n')) => {
                self.lineNo = self.lineNo + 1;
                self.colNo = 0;
            },
            _ => self.colNo = self.colNo + 1
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
        self.assert_not_eof();

        let mut startPosition: Option<Position> = None;
        let mut endPosition: Option<Position> = None;

        println!("starting a chomp at text: {}", self.text());
        println!("index is: {}", self.index);
        println!("isEof is {}", self.isEof);
        println!("last valid index of code is {}", self.code.len() - 1);
        // todo I KNOW this can be simplified and cleaned up
        loop {
            let should_quit = match self.peek() {
                None => {
                    // This means, there IS no next character. EOF.
                    endPosition = Some(self.position());
                    // Still need to call next(), to fully put chomper into EOF state.
                    self.next();
                    true
                },
                Some(ch) => {
                    if char_quit(ch) || str_quit(self.text()) {
                        endPosition = Some(self.position());
                        true
                    } else {
                        println!("Not time to quit yet!");
                        if startPosition == None {
                            println!("setting start index for chomp at {}", self.index);
                            startPosition = Some(self.position());
                        }
                        self.next();
                        false
                    }
                }
            };

            if should_quit {
                println!("Just about to create ChompResult");
                println!("startPosition is: {}", startPosition);
                println!("endPosition is: {}", endPosition);

                if startPosition == None {return None;}
                let cr = Some(ChompResult { span: Span { startPos: startPosition.unwrap(), endPos: endPosition.unwrap() },
                                            hitEof: self.isEof });

                println!("Full chomp result is: {}", cr);
                return cr;
            }
        }
    }

    pub fn value(&self, chompResult: ChompResult) -> &'ci str {
        self.code.slice(chompResult.span.startPos.index, chompResult.span.endPos.index)
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
        assert_eq!(cr.span.startPos.lineNo, 1);
        assert_eq!(cr.span.startPos.colNo, 0);

        assert_eq!(cr.span.endPos.lineNo, 4);
        assert_eq!(cr.span.endPos.colNo, 15);
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
    #[should_fail]
    fn chomp_should_return_none_if_youre_already_at_eof_when_you_call_it() {
        let code = "40";
        let mut chomper = Chomper::new(code);

        let chomper_borrow = &mut chomper;

        let result = chomper_borrow.chomp (|_| { false}).unwrap();
        assert_eq!(chomper_borrow.value(result), "40");

        chomper_borrow.chomp(|_| { false });
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
        assert_eq!(cr.span.startPos.index, 0);
        assert_eq!(cr.span.endPos.index, 8);
        assert_eq!(chomper.isEof, false);
    }

    #[test]
    fn chomp_till_str_should_work_when_there_is_no_match() {
        let code = "This is some text";
        let mut chomper = Chomper::new(code);
        let cr = chomper.chomp_till_str(|str| str.starts_with("XXXXXXX")).unwrap();
        println!("the cr is: {}", cr);
        assert_eq!(chomper.value(cr), "This is some text");
        assert_eq!(cr.span.startPos.index, 0);
        assert_eq!(cr.span.endPos.index, 17);
        assert_eq!(chomper.isEof, true);
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
        assert_eq!(combined.span.startPos.index, 0);
        assert_eq!(combined.span.endPos.index, 6);
        assert_eq!(chomper.isEof, true);
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
        assert_eq!(combined.span.startPos.index, 0);
        assert_eq!(combined.span.endPos.index, 6);
        assert_eq!(chomper.isEof, true);
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
        assert_eq!(combined.span.startPos.index, 0);
        assert_eq!(combined.span.endPos.index, 6);
        assert_eq!(chomper.isEof, true);
    }
}
