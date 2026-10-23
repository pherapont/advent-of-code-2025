pub fn get_voltage(battary: Vec<u32>) -> u32{
    battary[0]
}


#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn get_first() {
        let battary = vec![1, 2, 3];
        let res = get_voltage(battary);
        assert_eq!(res, 1);
    }
}
