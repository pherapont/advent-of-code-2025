#[derive(PartialEq)]
enum Place {
    Roll,
    Empty,
}

pub fn scan_field(field: Vec<Vec<&Place>>) -> u16 {
    42
}

// Функция возвращает true, если количество рулонов рядом меньше 4
fn check_window(field: &[Vec<&Place>], r: usize, c: usize) -> bool {
    let rows = field.len();
    // if rows == 0 {return?}
    let cols = field[0].len();
    let mut result = 0;

    let r_start = r.saturating_sub(1);
    let r_end = (r + 1).min(rows - 1);
    let c_start = c.saturating_sub(1);
    let c_end = (c + 1).min(cols - 1);
    for nr in r_start..=r_end {
        for nc in c_start..=c_end {
            if nr == r && nc == c {
                continue;
            }
            if field[nr][nc] == &Place::Roll {
                result += 1;
            }
        }
    }
    result < 4
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn less_than_4_rolls() {
        let r = Place::Roll;
        let e = Place::Empty;
        let row1 = vec![&e, &e, &r];
        let row2 = vec![&r, &e, &e];
        let row3 = vec![&e, &r, &e];
        let field = [row1, row2, row3];
        assert!(check_window(&field, 1, 1));
    }

    #[test]
    fn more_than_4_rolls() {
        let r = Place::Roll;
        let e = Place::Empty;
        let row1 = vec![&e, &r, &r];
        let row2 = vec![&r, &e, &r];
        let row3 = vec![&e, &r, &e];
        let field = [row1, row2, row3];
        assert!(!check_window(&field, 1, 1));
    }

    #[test]
    fn ecual_4_rolls() {
        let r = Place::Roll;
        let e = Place::Empty;
        let row1 = vec![&e, &r, &e];
        let row2 = vec![&r, &e, &r];
        let row3 = vec![&e, &r, &e];
        let field = [row1, row2, row3];
        assert!(!check_window(&field, 1, 1));
    }

    #[test]
    fn roll_in_corner() {
        let r = Place::Roll;
        let e = Place::Empty;
        let row1 = vec![&e, &r, &e];
        let row2 = vec![&r, &e, &r];
        let row3 = vec![&e, &r, &e];
        let field = [row1, row2, row3];
        assert!(check_window(&field, 0, 2));
    }

    #[ignore]
    #[test]
    fn only_one_row() {
        let r = Place::Roll;
        let e = Place::Empty;
        let row = vec![&r, &r, &r, &e, &e, &e, &r, &r];
        let field = vec![row];
        let result = 8;
        assert_eq!(scan_field(field), result);
    }
}
