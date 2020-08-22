// // This file demonstrates the overloading workaround when the root item needs a lifetime annotation.

// #[derive(Debug)]
// pub struct MagicNumber<'m> {
//     irrelevant_slice_that_needs_lifetime: &'m str,
//     value: u64
// }

// // During this process, don't get confused. THERE MUST BE ONE AND ONLY ONE IMPLEMENTATION OF ADD!
// // impl<'i, R: CanBeAddedToMagicNumber>  Add<R, MagicNumber<'i>> for MagicNumber<'i> {
// //     fn add<'g>(&'g self, rhs: &'g R) -> MagicNumber<'g> {
// //         rhs.add_to_magic_number(self)
// //     }
// // }

// // trait CanBeAddedToMagicNumber {
// //     fn add_to_magic_number<'h>(&'h self, lhs: &'h MagicNumber) -> MagicNumber<'h>;
// // }

// // impl<'j> CanBeAddedToMagicNumber for MagicNumber<'j> {
// //     fn add_to_magic_number<'k>(&'k self, lhs: &'k MagicNumber) -> MagicNumber<'k> {
// //         MagicNumber { value: lhs.value + self.value, irrelevant_slice_that_needs_lifetime: lhs.irrelevant_slice_that_needs_lifetime }
// //     }
// // }

// // impl<'m> CanBeAddedToMagicNumber for Option<MagicNumber<'m>> {
// //     fn add_to_magic_number<'n>(&'n self, lhs: &'n MagicNumber) -> MagicNumber<'n> {
// //         if self.is_none() { return *lhs; }
// //         lhs + self.unwrap()
// //     }
// // }

// fn main() {
//     // let one = MagicNumber { value: 40, irrelevant_slice_that_needs_lifetime: "hey" };
//     // let two = MagicNumber { value: 2, irrelevant_slice_that_needs_lifetime: "hey" };
//     // let result = one + two;
//     // println!("ignore the slice value of {}", result.irrelevant_slice_that_needs_lifetime); // prevents compiler warning
//     // println!("result: {}", result);
//     // assert_eq!(result.value, 42);

//     // let three : Option<MagicNumber> = None;

//     // let option_result = result + three;
//     // println!("option result: {}", option_result);
//     // assert_eq!(option_result.value, 42);

//     let r = 42.foo_it(2);
//     println!("got {}", r);
//     assert_eq!(84, r.some_number);

//     let ddd = 'c'.foo_it(2);
//     println!("got {}", ddd);
//     assert_eq!(22, ddd.some_number);
// }

// #[derive(Debug)]
// pub struct Foo {
//     some_number: u64
// }

// trait Fooable {
//     fn foo_it(&self, number: u64) -> Foo;
// }

// impl Fooable for u64 {
//     fn foo_it(&self, number: u64) -> Foo {
//         Foo { some_number: *self * number }
//     }
// }

// impl Fooable for char {
//     fn foo_it<'xxx>(&'xxx self, number: u64) -> Foo<'xxx> {
//         Foo { some_number: if *self == 'c' { 22 } else { 100 } }
//     }
// }

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
//
//  Let's go back to the beginning again. In my case, I'm adding 2 magic numbers. They BOTH have the SAME slice (which is the source
//   code being lexed -- even though this simplified example has nothing to do with source code or lexing). The truth is, that
//   source code lives longer than EITHER the lhs or the rhs. I COULD get rid of the slice in the lexer. Each ChompResult would
//   have indexes to start and end of the text, but would not have the actual text they were pointing to. You could easily go
//   get any slice if/when you need it. But I really like the idea of each ChompResult having its slice. Plus, I WANT to slog through
//   these hard lifetime questions.
//
//  So, the result of adding 2 MagicNumbers -- how long should it live. Truth is, I can get the code from EITHER ONE of them, so
//   my return value has to be tied to the lifetime of BOTH of them. That IS limiting, because I don't really care about the lifetimes
//   of the lhs and the rhs. I really care about the lifetime of the "source code". But, let's try to solve it with tying the
//   lifetime to both of them.
//
//  In that case, the **trait itself** would dictate that the fn require the lhs and rhs to have the same lifetime, which would
//   also be the lifetime of the return value. Let's try that.
//
//  OK, after sleeping on this last night, one thing is imminently clear: with Rust, you're going to want to limit the things that
//   have references. Like in this case. Both my MagicNumber and my ChompResult would be GREATLY simplified by not carrying around
//   the slice, and they can TOTALLY live WITHOUT that slice. And that is the right call in Rust (at least in its present state).
//   And it is probably the right call in C++ or any manual memory managemenet language. I think the borrow checker is exposing
//   the complexity of carrying around borrowed refs, whereas in C++, the complexity would still be there, but hidden. I'd only
//   find it months down the road when it bit me in the ass because I tried to use a ChompResult after I'd already cleared out
//   the underlying code.
//
//  If you want a ChompResult to hold onto its slice, then it MUST have a PERMANENT TETHER back to the actual code itself (since
//   that's what the slice refers to or borrows). And the
//   lifetime is what expresses that permanent tether. If you can abandon the permanent tether, but go back to the code and create
//   the slice any time you want, then you get way more flexibility as a result. I guess, a borrowed reference IS a permanent
//   tether, but languages without a borrow checker make that tether invisible.
//
//  So, as a thorough learning exercise, let me continue with this example until I get it all unravelled, and then, abandon it, and
//   go back and let me ChompResult become FREE!
//
//  So, the key question at this point is, if there's a Trait called "foo" with one function called "bar" that takes an int and returns
//   a bool, and you want to provide an impl of that trait ... can you do it with a function called "bar" that has LIFETIME PARAMS?
//   Or does the lifetime param modify your signature in such a way that you no longer qualify as implementing the trait? I suspect
//   it makes your sig no longer qualify. (Actually, to be more precise, it is not merely the presence of lifetime params. Those
//   are always there, even if you don't specify them. But tying the return value lifetime to a fn param lifetime probably would
//   disqualify your sig as a match for the trait). So I'm going to try this out above.
