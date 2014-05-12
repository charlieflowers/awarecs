// This is just me warming up with rust.

fn main() {
    use std::io::File;
    use std::str;

    let file_bytes = File::open(&Path::new("charlie-to-parse.txt")).read_to_end().unwrap();

    let contents = file_bytes.as_slice();
    println!("The slice is {}", contents);

    let string_contents = str::from_utf8(contents).unwrap();

    let first_char = string_contents[0];

    println!("The first char is {}", first_char);

    let second_char = string_contents[1];
    println!("the second char is {}", second_char);

    println!("Hello! I'm going to parse {}", string_contents);

    match first_char as char {
        '4' => println!("Yes, the first char matches '4'"),
        _   => fail!("No, first char does not match '4'")
    }

    for index in range(0u, string_contents.len()) {
        let next_char = string_contents[index] as char;
        println!("Next char is {}", next_char);
    }

    let index : &mut uint = &mut 0;

    loop {
        if *index >= string_contents.len()  { break; }
        eat_whitespace(string_contents, index);
        let next_char = string_contents[*index] as char;
        println!("char {} is {}.", next_char, *index);

        let token = match next_char {
            num if num.is_digit() => get_number(string_contents, index),
            '+' | '-' => get_operator(string_contents, index),
            _ => {fail!("unable to match char {} at index {}", next_char, *index)}
        };

        println!("Got token: {}", token);
        println!("After getting token, index is {}", *index);
    }
}

fn eat_whitespace(string_contents : &str, index : &mut uint) {
    let mut count = 0;
    loop {
        if *index >= string_contents.len() {break};
        if ! (string_contents [*index] as char).is_whitespace() {
            break;
        } else {
            count = count+1;
            *index = *index+1
        }
    }

    println!("Ate {} chars of whitespace.", count);
}

fn get_number(string_contents : &str, index : &mut uint) -> ~str{
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

fn get_operator(string_contents : &str, index : &mut uint) -> ~str {
    let mut operator = "".to_owned();
    loop {
        let ch = string_contents[*index] as char;
        if ch.is_whitespace() { return "Operator: ".to_owned() + operator; }
        if ch != '+' && ch != '-' { fail!("I thought I was parsing an operator, but I found this in it: {}", ch)}
        operator = operator + std::str::from_char(ch);
        *index = *index + 1;
        if *index >= string_contents.len() { fail!("Inside get_operator, we ran past end of parser input and were planning to keep going.");}
    }
}
