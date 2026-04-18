use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::day1::cyclic_calculator::{CyclicCalculator, Task};

pub mod day1;

fn main() {
    run("src/day1/input.txt");
}

fn run(path: &str) {
    let file = File::open(path).expect("Can't open the file");
    let reader = BufReader::new(file);
    let mut calc = CyclicCalculator::new(50, 100);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let task = Task::task_from_string(line);
                calc.complete_task(task);
                println!(
                    "Current position: {}, nuls count {}",
                    calc.get_pos(),
                    calc.get_nuls_count()
                );
            }
            Err(_) => {
                panic!("Can't read line");
            }
        }
    }
    println!("Finish position: {}", calc.get_pos());
    println!("Nuls count: {}", calc.get_nuls_count());
}
