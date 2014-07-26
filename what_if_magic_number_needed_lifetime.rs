// This file demonstrates the overloading workaround when the root item needs a lifetime annotation.

#[deriving(Show)]
pub struct MagicNumber<'m> {
    irrelevant_slice_that_needs_lifetime: &'m str,
    value: uint
}

// During this process, don't get confused. THERE MUST BE ONE AND ONLY ONE IMPLEMENTATION OF ADD!
impl<R: CanBeAddedToMagicNumber>  Add<R, MagicNumber> for MagicNumber {
    fn add(&self, rhs: &R) -> MagicNumber {
        rhs.add_to_magic_number(self)
    }
}

trait CanBeAddedToMagicNumber {
    fn add_to_magic_number(&self, lhs: &MagicNumber) -> MagicNumber;
}

impl CanBeAddedToMagicNumber for MagicNumber {
    fn add_to_magic_number(&self, lhs: &MagicNumber) -> MagicNumber {
        MagicNumber { value: lhs.value + self.value }
    }
}

impl CanBeAddedToMagicNumber for Option<MagicNumber> {
    fn add_to_magic_number(&self, lhs: &MagicNumber) -> MagicNumber {
        if self.is_none() { return *lhs; }
        lhs + self.unwrap()
    }
}

fn main() {
    let one = MagicNumber { value: 40 };
    let two = MagicNumber { value: 2 };
    let result = one + two;
    println!("ignore the slice value of {}", result.irrelevant_slice_that_needs_lifetime); // prevents compiler warning
    println!("result: {}", result);
    assert_eq!(result.value, 42);

    let three : Option<MagicNumber> = None;

    let option_result = result + three;
    println!("option result: {}", option_result);
    assert_eq!(option_result.value, 42);
}

// Here's how the trnsformation unfolded step by step:
// 1. I added the slice to MagicNumber, and a line in main to print it out so we wouldn't get compiler warning.
