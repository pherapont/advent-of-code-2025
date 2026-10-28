use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use day3::get_voltage;

fn main() -> io::Result<()> {
    let path = Path::new("src/input.txt");
    let f = File::open(&path)?;
    let reader = BufReader::new(f);
    let mut battary: Vec<u32> = Vec::new();
    let mut voltage: u32 = 0;

    for line in reader.lines() {
        battary.clear();
        let content = line.expect("Can't read line");
        println!("{:?}", content);
        for &byte in content.as_bytes() {
            let digit = byte - b'0';
            battary.push(digit as u32);
        }
        voltage += get_voltage(&battary);
        println!("{}", voltage);
    }

    Ok(())
}
