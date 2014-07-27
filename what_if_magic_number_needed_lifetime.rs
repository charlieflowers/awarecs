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

impl<'b> CanBeAddedToMagicNumber for MagicNumber<'b> {
    fn add_to_magic_number(&self, lhs: &MagicNumber) -> MagicNumber {
        MagicNumber { value: lhs.value + self.value, irrelevant_slice_that_needs_lifetime: "hey" }
    }
}

impl<'c> CanBeAddedToMagicNumber for Option<MagicNumber<'c>> {
    fn add_to_magic_number(&self, lhs: &'c MagicNumber) -> MagicNumber<'c> {
        if self.is_none() { return *lhs; }
        lhs + self.unwrap()
    }
}

fn main() {
    let one = MagicNumber { value: 40, irrelevant_slice_that_needs_lifetime: "hey" };
    let two = MagicNumber { value: 2, irrelevant_slice_that_needs_lifetime: "hey" };
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
//
//        YES, that worked. Down to 2 errors, and so far everything makes sense to me.
//
//     c. Next error is on line 20, after the "for". I believe this is simply a matter of adding 'b to MagicNumber after "for", which
//         will give me an undeclared lifetime error for 'b. Then, I'll add 'b right after the impl to declare it, and I'll be
//         down to only one error. YES: IT PLAYED OUT EXACTLY LIKE THAT!
//
//     d. Next error is line 26, again, after the "for". I will fix it using 'c, exactly the same way as in step c.
//         YES, that worked.
//
// 4. That seems to have made the first "wave" of errors happy and let the compiler get further. Now I have these errors:
//
    // :22:9: 22:54 error: missing field: `irrelevant_slice_that_needs_lifetime`
    // :22         MagicNumber { value: lhs.value + self.value }
    //             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // :27:5: 30:6 note: consider using an explicit lifetime parameter as shown: fn add_to_magic_number<'a>(&self, lhs: &MagicNumber<'a>) -> MagicNumber<'a>
    // :27     fn add_to_magic_number(&self, lhs: &MagicNumber) -> MagicNumber {
    // :28         if self.is_none() { return *lhs; }
    // :29         lhs + self.unwrap()
    // :30     }
    // :28:36: 28:40 error: mismatched types: expected `MagicNumber<'_>` but found `MagicNumber<'_>` (lifetime mismatch)
    // :28         if self.is_none() { return *lhs; }
    //                                        ^~~~
    // :34:15: 34:40 error: missing field: `irrelevant_slice_that_needs_lifetime`
    // :34     let one = MagicNumber { value: 40 };
    //                   ^~~~~~~~~~~~~~~~~~~~~~~~~
    // :35:15: 35:39 error: missing field: `irrelevant_slice_that_needs_lifetime`
    // :35     let two = MagicNumber { value: 2 };
    //                                                          ^~~~~~~~~~~~~~~~~~~~~~~~
    // error: aborting due to 4 previous errors

//   This is SUPER CONFUSING. Consider the first error and the first note. It tells me "missing field". Well that actually makes
//    sense. I'm instantiating the struct, but I omitted a field. So WHY IN THE HELL is it followed about a note about LIFETIMES?
//    The note does not seem related to the error message at all!
//   I'm going to fix the error I understand ... actually, all 3 of the "missing field" errors, then see what we've got. That's
//    line 22, line 34, and line 35.
//
//   YES. This left me with one error, as follows:
//
        // :27:5: 30:6 note: consider using an explicit lifetime parameter as shown: fn add_to_magic_number<'a>(&self, lhs: &MagicNumber<'a>) -> MagicNumber<'a>
        // :27     fn add_to_magic_number(&self, lhs: &MagicNumber) -> MagicNumber {
        // :28         if self.is_none() { return *lhs; }
        // :29         lhs + self.unwrap()
        // :30     }
        // :28:36: 28:40 error: mismatched types: expected `MagicNumber<'_>` but found `MagicNumber<'_>` (lifetime mismatch)
        // :28         if self.is_none() { return *lhs; }
        //                                                                               ^~~~
        // error: aborting due to previous error
//
//   As you can see, the mysterious "note" is not so mysterious. It merely PRECEDES the error message it applies to. So line
//   28 has some kind of "lifetime mismatch". We are trying to return *lhs. Let's think about this. Basically, in this case,
//   I have to add 2 MagicNumbers. I will either return their sum, or return only lhs. My return value needs to live NO LONGER
//   than BOTH of my inputs. Why? That WOULD NOT be a requirement without the slice! I'd just happily allocate a new MagicNumber with
//   the correct "value" field. But NOW, my resulting MagicNumber will have that slice in it. Whose memory is it derived from?
//   Well, depending on how I choose to implement "Add", it could be either of them. Conceptually, in this case, it is BOTH of them.
//   I think it would be equally correct to do either of these:
//     a. Since "self" on line 28 already has a lifetime of 'c (due to the "for" on line 26), just say my return type on line 27
//         lives for 'c.
//      OR
//     b. Add a lifetime parameter 'e to the fn add_to_magic_number on line 27. Say that BOTH input parameters, self and lhs, live
//         for 'e, and say my return value lives for 'e.
//   Of course, like a truly ocd basket case, I will try BOTH.
//
//    RESULTS OF TRYING APPROACH a:
//
        // :27:5: 30:6 note: consider using an explicit lifetime parameter as shown: fn add_to_magic_number(&self, lhs: &MagicNumber<'c>) -> MagicNumber<'c>
        // :27     fn add_to_magic_number(&self, lhs: &MagicNumber) -> MagicNumber<'c> {
        // :28         if self.is_none() { return *lhs; }
        // :29         lhs + self.unwrap()
        // :30     }
        // :28:36: 28:40 error: mismatched types: expected `MagicNumber<'c>` but found `MagicNumber<'_>` (lifetime mismatch)
        // :28         if self.is_none() { return *lhs; }
        //                                                                               ^~~~
        // :27:5: 30:6 error: method `add_to_magic_number` has an incompatible type for trait: expected concrete lifetime, but found bound lifetime parameter
        // :27     fn add_to_magic_number(&self, lhs: &MagicNumber) -> MagicNumber<'c> {
        // :28         if self.is_none() { return *lhs; }
        // :29         lhs + self.unwrap()
        // :30     }
        // :27:73: 30:6 note: expected concrete lifetime is the lifetime 'c as defined on the block at 27:72
        // :27     fn add_to_magic_number(&self, lhs: &MagicNumber) -> MagicNumber<'c> {
        // :28         if self.is_none() { return *lhs; }
        // :29         lhs + self.unwrap()
        // :30     }
        // error: aborting due to 2 previous errors
//
// This makes sense to me, because at one point, I return lhs. But nothing I've said has tied the lifetime of lhs to c. So let's do
//  that.
//
// I did that, and got the following error:
//
        // :27:5: 30:6 error: method `add_to_magic_number` has an incompatible type for trait: expected concrete lifetime, but found bound lifetime parameter
        // :27     fn add_to_magic_number(&self, lhs: &'c MagicNumber) -> MagicNumber<'c> {
        // :28         if self.is_none() { return *lhs; }
        // :29         lhs + self.unwrap()
        // :30     }
        // :27:76: 30:6 note: expected concrete lifetime is the lifetime 'c as defined on the block at 27:75
        // :27     fn add_to_magic_number(&self, lhs: &'c MagicNumber) -> MagicNumber<'c> {
        // :28         if self.is_none() { return *lhs; }
        // :29         lhs + self.unwrap()
        // :30     }
        // error: aborting due to previous error
//
// This is actually starting to make sense to me, at least partly. It is saying that my add_to_magic_number signature no
//  longer matches that required by the trait. Adding 'c to lhs apparently invalidated it. But, what's this business about
//  "bound" and "concrete"? I SUSPECT that a "bound" lifetime param is one attached to an input param. But I dunno.
//
//  I do find some relevant insight here: http://stackoverflow.com/questions/24847331/rust-lifetime-error-expected-concrete-lifetime-but-found-bound-lifetime
//
//  What I conclude is that I'm still not accurately telling the compiler what my expectations are about the lifetimes. Ultimately,
//   if I'm adding 2 MagicNumbers, because they're both pointing to this same slice, my return value cannot outlive the slice. If
//   I make sure my return value doesn't outlive EITHER ONE of my inputs, that should be accomplished just fine. In other words,
//   the lifetime of my return value must be tied to the lifetimes of BOTH my inputs. And, if that's what I expect, then SAY so
//   DIRECTLY on the initial trait to begin with! Then, whomever implements the trait will be forced to comply with it.
