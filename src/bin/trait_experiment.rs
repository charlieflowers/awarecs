#[deriving(Show)]
pub struct MagicNumber {
    value: uint
}

// This works, by itself
// impl Add<MagicNumber, MagicNumber> for MagicNumber {
//     fn add(&self, rhs: &MagicNumber) -> MagicNumber {
//         MagicNumber { value: self.value + rhs.value }
//     }
// }

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
    println!("result: {}", result);
    assert_eq!(result.value, 42);

    let three : Option<MagicNumber> = None;

    let option_result = result + three;
    println!("option result: {}", option_result);
    assert_eq!(option_result.value, 42);
}
