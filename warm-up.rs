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


}
