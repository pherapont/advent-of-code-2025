use std::fs::File;
use std::io::BufReader;

enum Directs {
    Left,
    Right,
}

struct Task {
    dir: Directs,
    count: u32,
}

pub fn run_dial(tasks: BufReader<File>) -> u32 {
    for line in tasks {
        match line[0] {
            'L' =>
        }
    }
}
