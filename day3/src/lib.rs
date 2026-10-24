pub fn get_voltage(battary: Vec<u32>) -> u32{
    let mut first: u32 = 0;
    let mut second: u32 = 0;
    for (index, &cell)  in battary.iter().enumerate(){
        if (cell > first) && (index < battary.len()-1){
            first = cell;
            second = 0;
        }
        else if cell > second {
            second = cell;
        }
    }
    first * 10 + second
}


#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn get_v1() {
        let battary = vec![1, 2, 3];
        let res = get_voltage(battary);
        assert_eq!(res, 23);
    }
}
