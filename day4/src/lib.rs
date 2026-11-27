#[derive(PartialEq, Debug)]
pub enum Place {
    Roll,
    Empty,
}

pub fn parse_line(line: String) -> Vec<Place> {
    let mut row: Vec<Place> = Vec::new();
    for c in line.bytes() {
        match c {
            b'.' => {
                row.push(Place::Empty);
            }
            b'@' => {
                row.push(Place::Roll);
            }
            _ => {
                continue;
            }
        }
    }
    row
}

pub fn scan_field(field: &Vec<Vec<Place>>) -> (u16, Vec<Vec<Place>>) {
    let mut res = 0;
    let mut new_field: Vec<Vec<Place>> = Vec::new();
    for (nr, row) in field.iter().enumerate() {
        for (nc, cell) in row.iter().enumerate() {
            if cell == &Place::Roll && check_window(&field, nr, nc) {
                new_field[nr][nc] = Place::Empty;
                res += 1;
            } else if cell == &Place::Roll {
                new_field[nr][nc] = Place::Roll;
            } else {
                new_field[nr][nc] = Place::Empty;
            }
        }
    }
    (res, new_field)
}

// Функция возвращает true, если количество рулонов рядом меньше 4
fn check_window(field: &Vec<Vec<Place>>, r: usize, c: usize) -> bool {
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
            if field[nr][nc] == Place::Roll {
                result += 1;
            }
        }
    }
    result < 4
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn parse_simple_line() {
        let line = "...@@@..";
        let row = vec![
            Place::Empty,
            Place::Empty,
            Place::Empty,
            Place::Roll,
            Place::Roll,
            Place::Roll,
            Place::Empty,
            Place::Empty,
        ];
        assert_eq!(parse_line(line.to_string()), row);
    }
}
