use crate::day1::cyclic_calculator::CyclicCalculator;

pub mod day1;

fn main() {
    let mut calc = CyclicCalculator::new(50, 100);
    calc.add(22);
    calc.add(10);
    calc.reduce(3);
    println!("Calculus: {}", calc.get_pos());
}
