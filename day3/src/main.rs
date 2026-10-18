//TODO First: get strings from text_input.txt
//      Second: 
//
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("test_input.txt");
    let f = File::open(&path);
    let reader = BufReader::new(f);
    let mut buf = String::new();

    while reader.read_line(&mut buf)? > 0 {
       print!("{}", buf);
       buf.clear();
    }

    println!("Hello");

    Ok(())
}
