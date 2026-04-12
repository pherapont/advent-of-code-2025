struct CyclicCalculator {
    pos: u8,
    reg: u8,
}

impl CyclicCalculator {
    pub fn new(initial_pos: u8, regularity: u8) -> Self {
        CyclicCalculator {
            pos: initial_pos,
            reg: regularity,
        }
    }
}
