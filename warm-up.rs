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

    // let contents = str::from_utf8(File::open (&Path::new("charlie-to-parse.txt")).read_to_end ().unwrap ().as_slice ());

    // let codeToParse = str::from_utf8(contents.unwrap().as_slice());

    println!("Hello! I'm going to parse {}", string_contents);

    match first_char as char {
        '4' => println!("Yes, the first char matches '4'"),
        _   => fail!("No, first char does not match '4'")
    }

    for index in range(0u, string_contents.len()) {
        let next_char = string_contents[index] as char;
        println!("Next char is {}", next_char);
    }

    let mut index = 0;

    loop {
        eat_whitespace(string_contents, index);
        if index >= string_contents.len()  { break; }
        let next_char = string_contents[index] as char;
        println!("char {} is {}.", next_char, index);

        let token = match next_char {
            num if num.is_digit() => get_number(string_contents, index),
            op if op == '+' || op == '-' => get_operator(string_contents, index, op),
            _ => {fail!("unable to match char {} at index {}", next_char, index)}
        };

        println!("Got token: {}", token);
    }

    println!("Congrats on using loop successfully.");
}

fn eat_whitespace(string_contents : &str, mut index : uint) {
    let mut count = 0;
    loop {
        if (string_contents [index] as char).is_whitespace() {
            count = count+1;
            index = index+1;
        } else {
            println!("Ate {} chars of whitespace.", count);
            return;
        }
    }
}

fn get_number(string_contents : &str, mut index : uint) -> ~str{
    let mut value = "".to_owned();
    loop {
        let ch = string_contents[index] as char;
        if ch.is_whitespace() { return "Number: ".to_owned() + value; }
        if ! ch.is_digit() { fail!("Found a {} right in the middle of an expected number. Can't do that.", ch)}
        value = value + std::str::from_char(ch);
        index = index + 1;
        if index >= string_contents.len() { fail!("Inside get_number, we ran past end of parser input and were planning to keep going.");}
    }
}

fn get_operator(string_contents : &str, mut index : uint, operator_char : char) -> ~str {
    fail!("get_operator not impleemnted yet, but index is {} and char is {}", index, operator_char);
}
