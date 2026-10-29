const VOLT_LEN: usize = 12;
struct CELL {
    v: u32,
    pos: usize,
}

pub fn get_voltage(battary: &Vec<u32>) -> u32 {
    let mut first: u32 = 0;
    let mut second: u32 = 0;
    for (index, &cell) in battary.iter().enumerate() {
        if (cell > first) && (index < battary.len() - 1) {
            first = cell;
            second = 0;
        } else if cell > second {
            second = cell;
        }
    }
    first * 10 + second
}

// теперь из большой строки надо получить 12 цифр
// Идея: первую цифра должна быть в срезе 0..l.len()-12
// это должна быть самая большая цифра и самая ранняя - чтобы оставить больше возможностей следующим.
// 2. Вторую ищем в срезе i1..l.len()-11, по томуже принципу, что и первую.
pub fn get_max_voltage(battary: &Vec<u32>) -> u64 {
    let mut ifirst: usize = 0;
    let mut ilast: usize = battary.len() - VOLT_LEN;
    let mut voltage_map: Vec<u32> = vec![0; 12];
    let mut res: u64 = 0;
    let base: u64 = 10;

    for volt in 0..VOLT_LEN {
        let mut max_cell = CELL { pos: 0, v: 0 };
        for (index, cell) in battary[ifirst..ilast].iter().enumerate() {
            if cell >= &max_cell.v {
                max_cell.v = *cell;
                max_cell.pos = index + ifirst;
            }
        }
        ifirst = max_cell.pos;
        ilast += 1;
        voltage_map[volt] = max_cell.v;
        res += max_cell.v as u64 * base.pow((12 - volt) as u32);
    }

    println!("{:?}", voltage_map);

    res
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get_v1() {
        let battary = vec![1, 2, 3];
        let res = get_voltage(&battary);
        assert_eq!(res, 23);
    }
}
