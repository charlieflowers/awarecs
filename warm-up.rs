// This is just me warming up with rust.
use std::fmt;

#[deriving(Show)]
enum TokenTag {
    Number,
    Whitespace,
    Operator,
    Herecomment,
    Comment
}

#[deriving(Show)]
struct Token {
    tag: TokenTag,
    value: ~str,
    length: uint,
    index: uint
}

impl Token {
    fn new(tag: TokenTag, value: ~str, length: uint, index: uint) -> Token {
        Token {tag:tag, value:value, length:length, index:index}
    }
}

fn main() {
    use std::io::File;
    use std::str;

    let file_bytes = File::open(&Path::new("charlie-to-parse.txt")).read_to_end().unwrap();

    let contents = file_bytes.as_slice();
    println!("The slice is {}", contents);

    let string_contents = str::from_utf8(contents).unwrap();

    let index : &mut uint = &mut 0;

    loop {
        if *index >= string_contents.len()  { break; }
        let next_char = string_contents[*index] as char;
        println!("char {} is {}.", next_char, *index);

        let token = match next_char {
            ws if ws.is_whitespace() => get_whitespace(string_contents, index),
            num if num.is_digit() => get_number(string_contents, index),
            '+' | '-' => get_operator(string_contents, index),
            '#' => get_comment(string_contents, index),
            _ => {fail!("unable to match char {} at index {}", next_char, *index)}
        };

        println!("Got token: {}", token);
        println!("After getting token, index is {}", *index);
    }
}

fn get_whitespace(string_contents : &str, index : &mut uint) -> Token {
    let mut value = "".to_owned();
    loop {
        if *index >= string_contents.len() {break};
        let ch = string_contents[*index] as char;
        if ! ch.is_whitespace() {
            break;
        } else {
            value = value + std::str::from_char(ch);
            *index = *index+1
        }
    }

    if value.len() == 0  { fail!("You are not supposed to call get_whitespace unless you know you got some. But I found zero characters of whitespace.")}

    return Token::new(Whitespace, value, value.len(), *index - value.len() )
}

fn get_number(string_contents : &str, index : &mut uint) -> Token {
    let mut value = "".to_owned();
    loop {
        let ch = string_contents[*index] as char;
        if ch.is_whitespace() { return "Number: ".to_owned() + value; }
        if ! ch.is_digit() { fail!("Found a {} right in the middle of an expected number. Can't do that.", ch)}
        value = value + std::str::from_char(ch);
        *index = *index + 1;
        if *index >= string_contents.len() { fail!("Inside get_number, we ran past end of parser input and were planning to keep going.");}
    }
}

fn get_operator(string_contents : &str, index : &mut uint) -> Token {
    let mut result = "".to_owned();
    loop {
        let ch = string_contents[*index] as char;
        if ch.is_whitespace() { return "Operator: ".to_owned() + result; }
        if ch != '+' && ch != '-' { fail!("I thought I was parsing an result, but I found this in it: {}", ch)}
        result = result + std::str::from_char(ch);
        *index = *index + 1;
        if *index >= string_contents.len() { fail!("Inside get_operator, we ran past end of parser input and were planning to keep going.");}
    }
}

fn get_comment(string_contents : &str, index : &mut uint) -> Token {
    let first_ch = string_contents[*index] as char;
    if first_ch != '#' { fail!("I thought I was parsing a comment, but it starts with this: {}", first_ch)}
    let next_ch = string_contents[*index + 1] as char;
    if next_ch == '#' { return get_herecomment(string_contents, index); }
    let mut result = "".to_owned();
    loop {
        let ch = string_contents[*index] as char;
        result = result + std::str::from_char(ch);
        *index = *index + 1;
        if ch == '\n' { return "Comment: ".to_owned() + result; }
        if *index >= string_contents.len() { fail!("Inside get_comment, we ran past end of parser input and were planning to keep going.");}
    }
}

fn expect(string_contents : &str, index : &mut uint, expectation : &str) -> Token {
    let my_slice = string_contents.slice_from(*index);
    println!("expecting! At index {}, expecting {} from {}.", *index, expectation, my_slice);

    if ! my_slice.starts_with(expectation) { fail!("At index {}, expected {} but got \r\n {}.", *index, expectation, string_contents.slice_from(*index))}

    let mut result = "".to_owned();
    // todo charlie of course its crazy to append char by char
    for n in range(0, expectation.len()) {
        let actual = my_slice[n] as char;
        result = result + std::str::from_char(actual);
        *index = *index + 1;
    }
    return result;
}

fn get_herecomment(string_contents : &str, index : &mut uint) -> Token {
    let mut result = expect(string_contents, index, "###");
    // Just keep going no matter what, until you hit the end or find ###.
    loop {
        let ch = string_contents[*index] as char;
        result = result + std::str::from_char(ch);
        if ch == '#' {
            if string_contents[*index + 1] as char == '#' && string_contents[*index + 2] as char == '#' {
                result = result + expect(string_contents, index, "###");
                return "HERECOMMENT: ".to_owned() + result;
            }
        }
        *index = *index + 1;
        if *index >= string_contents.len() { fail!("Inside get_herecomment, we ran past end of parser input and were planning to keep going. The herecomment token we have so far is {}", result);}
    }
}

#[test]
fn whole_shebang_should_succeed() {
    main();
}
