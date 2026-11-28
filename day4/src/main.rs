use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use day4::{Place, parse_line, scan_field};

const FILE_MAP: &str = "src/input.txt";

fn main() -> io::Result<()> {
    let f = File::open(FILE_MAP)?;
    let reader = BufReader::new(f);
    let mut field: Vec<Vec<Place>> = Vec::new();
    let mut result: u16 = 0;

    for line in reader.lines() {
        let line = line?;
        let row = parse_line(line);
        field.push(row);
    }
    // todo field должно изменяться от итерации к итерацииЫ
    loop {
        let tmp: u16;
        (tmp, field) = scan_field(&field);
        if tmp != 0 {
            result += tmp;
        } else {
            break;
        }
    }

    println!("Result = {}.", result);
    Ok(())
}
