pub struct CyclicCalculator {
    pos: u16,
    reg: u16,
}

impl CyclicCalculator {
    pub fn new(initial_pos: u16, regularity: u16) -> Self {
        if initial_pos >= regularity {
            panic!("Position can't be lager periond!");
        }
        CyclicCalculator {
            pos: initial_pos,
            reg: regularity,
        }
    }

    pub fn add(&mut self, steps: u16) -> u16 {
        let mut res: u16 = self.pos + steps;
        if res > self.reg {
            res = res - self.reg;
        }
        self.pos = res;
        res
    }

    pub fn reduce(&mut self, steps: u16) -> u16 {
        let res = if self.pos < steps {
            self.pos + self.reg - steps
        } else {
            self.pos - steps
        };
        self.pos = res;
        res
    }
    pub fn get_pos(&self) -> u16 {
        self.pos
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
