const VOLT_LEN: usize = 12;

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
pub fn get_max_voltage(battary: &Vec<u32>) -> u32 {
    let mut ifirst: usize = 0;
    let mut ilast: usize = battary.len() - 1;
    let mut voltage_map: Vec<u8> = vec![0; 12];

    for volt in 0..VOLT_LEN {
        // TODO Заменить на поиск реального значения
        voltage_map[volt] = 8;
    }

    println!("{:?}", voltage_map);

    42
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
