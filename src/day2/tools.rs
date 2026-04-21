use std::fs;

pub fn run_tools(file_path: &str) -> u64 {
    let intervals = get_intervals(file_path);
    let intervals = parse_intervals(intervals);
    let invalid_ids = search_invalid_ids(intervals);
    let mut res: u64 = 0;
    for num in invalid_ids {
        res += num;
    }
    res
}

fn get_intervals(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file.");
    content
}

fn parse_intervals(content: String) -> Vec<(u64, u64)> {
    let mut res = Vec::new();
    for interval in content.trim().split(",") {
        let bounds_str: Vec<&str> = interval.split("-").collect();
        let mut bounds: Vec<u64> = Vec::new();
        for bound in bounds_str {
            let bound: u64 = bound.parse().expect("Wrong input data.");
            bounds.push(bound);
        }
        res.push((bounds[0], bounds[1]));
    }
    res
}

fn search_invalid_ids(data: Vec<(u64, u64)>) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    for record in data {
        let start = record.0;
        let end = record.1;
        for number in start..=end {
            if search_double_string(number.to_string()) {
                res.push(number);
            }
        }
    }
    res
}

fn search_double_string(line: String) -> bool {
    let bound = line.len() / 2;
    let head = &line[..bound];
    let tail = &line[bound..];
    head == tail
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn get_intervals_from_file() {
        let data = get_intervals("src/day2/test_input.txt");
        let result = "11-22,5-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862".to_string();
        assert_eq!(data.trim(), result);
    }

    #[test]
    fn parse_simple_intervals() {
        let data = "11-22,5-115,998-1012,1188511880-1188511890".to_string();
        let res: Vec<(u64, u64)> = vec![(11, 22), (5, 115), (998, 1012), (1188511880, 1188511890)];
        assert_eq!(parse_intervals(data), res);
    }

    #[test]
    fn double_string() {
        assert!(search_double_string("abcdabcd".to_string()));
    }

    #[test]
    fn not_double_string() {
        assert!(!search_double_string("abcdefgh".to_string()));
    }

    #[test]
    fn catch_repeated() {
        let data: Vec<(u64, u64)> = vec![(95, 105), (11, 22)];
        let res: Vec<u64> = vec![99, 11, 22];
        assert_eq!(search_invalid_ids(data), res);
    }
}
