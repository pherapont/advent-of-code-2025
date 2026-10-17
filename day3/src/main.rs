//TODO First: get strings from text_input.txt
//      Second: 
//
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("test_input.txt");
    let file = File::open(&path);
    let reader = BufReader::new(file);

    for line in reader.lines(){
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}
