use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};

pub fn read_input(path: String) -> BufReader<File> {
    let file = File::open(path).expect("Can't open the file");
    let reader = BufReader::new(file);
    reader
}
