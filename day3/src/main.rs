//TODO First: get strings from text_input.txt
//      Second: 
//
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("src/test_input.txt");
    let f = File::open(&path)?;
    let reader = BufReader::new(f);

    for line in reader.lines(){
        let content = line.expect("Can't read line");
        println!("{:?}", content);
        for &byte in content.as_bytes(){
            let digit = byte - b'0';
            print!("{}--", digit);
        }
            println!("");
    }

    Ok(())
}
