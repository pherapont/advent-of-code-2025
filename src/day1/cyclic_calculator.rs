#[derive(Debug)]
enum Directs {
    Left,
    Right,
}

pub struct Task {
    dir: Directs,
    count: u16,
}

impl Task {
    pub fn task_from_string(line: String) -> Task {
        let dir = match line.chars().nth(0) {
            Some('L') => Directs::Left,
            Some('R') => Directs::Right,
            Some(_) => panic!("Uncorrect task1"),
            None => panic!("Cant parse task1"),
        };
        let steps: u16 = line[1..].parse().expect("Can't parse number");
        println!("dir: {:?}, steps: {}", dir, steps);
        Task { dir, count: steps }
    }
}

pub struct CyclicCalculator {
    pos: u16,
    reg: u16,
    nuls_count: u16,
}

impl CyclicCalculator {
    pub fn new(initial_pos: u16, regularity: u16) -> Self {
        if initial_pos >= regularity {
            panic!("Position can't be lager periond!");
        }
        CyclicCalculator {
            pos: initial_pos,
            reg: regularity,
            nuls_count: 0,
        }
    }

    pub fn complete_task(&mut self, task: Task) {
        match task.dir {
            Directs::Left => self.pos = self.reduce(task.count),
            Directs::Right => self.pos = self.add(task.count),
        }
        self.check_nul();
    }

    pub fn get_pos(&self) -> u16 {
        self.pos
    }

    pub fn get_nuls_count(self) -> u16 {
        self.nuls_count
    }

    fn add(&mut self, steps: u16) -> u16 {
        let mut res: u16 = self.pos + steps % self.reg;
        if res >= self.reg {
            res = res - self.reg;
        }
        self.pos = res;
        res
    }

    fn reduce(&mut self, steps: u16) -> u16 {
        let res = if self.pos < steps {
            self.pos + self.reg - steps % self.reg
        } else {
            self.pos - steps % self.reg
        };
        self.pos = res;
        res
    }

    fn check_nul(&mut self) {
        if self.pos == 0 {
            self.nuls_count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_correct_calculator() {
        let calc = CyclicCalculator::new(50, 100);
        assert_eq!(calc.pos, 50);
        assert_eq!(calc.reg, 100);
    }

    #[test]
    #[should_panic]
    fn create_uncorrect_calculator() {
        let calc = CyclicCalculator::new(110, 100);
    }

    #[test]
    fn add_within_regularity() {
        let mut calc = CyclicCalculator::new(50, 100);
        let pos = calc.add(25);
        assert_eq!(pos, 75);
    }

    #[test]
    fn add_out_regularity() {
        let mut calc = CyclicCalculator::new(50, 100);
        let pos = calc.add(70);
        assert_eq!(pos, 20);
    }

    #[test]
    fn reduce_within_regularity() {
        let mut calc = CyclicCalculator::new(50, 100);
        let pos = calc.reduce(25);
        assert_eq!(pos, 25);
    }

    #[test]
    fn reduce_out_regularity() {
        let mut calc = CyclicCalculator::new(50, 100);
        let pos = calc.reduce(84);
        assert_eq!(pos, 66);
    }
}
