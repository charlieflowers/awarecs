// This file demonstrates the overloading workaround when the root item needs a lifetime annotation.

#[deriving(Show)]
pub struct MagicNumber<'m> {
    irrelevant_slice_that_needs_lifetime: &'m str,
    value: uint
}

// During this process, don't get confused. THERE MUST BE ONE AND ONLY ONE IMPLEMENTATION OF ADD!
impl<'a, R: CanBeAddedToMagicNumber>  Add<R, MagicNumber<'a>> for MagicNumber<'a> {
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
// 2. I got these compile errors:

    // :10:59: 10:70 error: wrong number of lifetime parameters: expected 1 but found 0
    // :10 impl<R: CanBeAddedToMagicNumber>  Add<R, MagicNumber> for MagicNumber {
    //                                                               ^~~~~~~~~~~
    // :10:42: 10:53 error: wrong number of lifetime parameters: expected 1 but found 0
    // :10 impl<R: CanBeAddedToMagicNumber>  Add<R, MagicNumber> for MagicNumber {
    //                                              ^~~~~~~~~~~
    // :20:34: 20:45 error: wrong number of lifetime parameters: expected 1 but found 0
    // :20 impl CanBeAddedToMagicNumber for MagicNumber {
    //                                      ^~~~~~~~~~~
    // :26:41: 26:52 error: wrong number of lifetime parameters: expected 1 but found 0
    // :26 impl CanBeAddedToMagicNumber for Option<MagicNumber> {
    //
    // error: aborting due to 4 previous errors

// Note, this is NOT every place that MagicNumber is used as a type. For example, line 11 says the fn returns type MagicNumber, and
//  that is not (yet at least) flagged as a compiler error.

// 3. So let's make these 4 errors happy, IN ORDER.
//     a. Addressing 1st error on line 10, after "for". I put lifetime 'a there, but then, compiler says:
            // :10:71: 10:73 error: use of undeclared lifetime name `'a`
            // :10 impl<R: CanBeAddedToMagicNumber>  Add<R, MagicNumber> for MagicNumber<'a> {

//        So, cannot put a lifetime there without "declaring" said lifetime. I believe this needs to go on the impl on line 10. Doing
//         that now. YES -- down to 3 errors.
//     b. Next error is ALSO on line 10, inside the type parameters for Add. It's pretty abstract to think about which lifetime
//        should go here. What is our return type? Well, really, we are going to return a MagicNumber that lives as long as
//        our *lhs*. Our lhs is our self param, which is the same type as the thing after "for". So I will use 'a.
