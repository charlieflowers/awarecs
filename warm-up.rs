// This is just me warming up with rust.

fn main() {
    use std::io::File;
    use std::str;

    let file_bytes = File::open(&Path::new("charlie-to-parse.txt")).read_to_end().unwrap();

    let contents = file_bytes.as_slice();
    println!("The slice is {}", contents);

    let string_contents = str::from_utf8(contents).unwrap();



    // let contents = str::from_utf8(File::open (&Path::new("charlie-to-parse.txt")).read_to_end ().unwrap ().as_slice ());

    // let codeToParse = str::from_utf8(contents.unwrap().as_slice());

    println!("Hello! I'm going to parse {}", string_contents);



}
