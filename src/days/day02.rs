use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_data<R: BufRead>(reader: R) -> Vec<Vec<i32>> {
    let mut data = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let row = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        data.push(row);
    }
    data
}

fn is_safe(report: &[i32]) -> bool {
    (report.windows(2).all(|w| w[0] < w[1]) || report.windows(2).all(|w| w[0] > w[1]))
        && report.windows(2).all(|w| (w[0] - w[1]).abs() <= 3)
}

fn part1(data: &Vec<Vec<i32>>) -> usize {
    let result = data.iter().map(|row| is_safe(row)).filter(|&b| b).count();
    result
}

fn and(a: &[bool], b: &[bool]) -> Vec<bool> {
    a.iter().zip(b).map(|(x, y)| x & y).collect()
}

fn part2(data: &Vec<Vec<i32>>) -> usize {
    fn is_safe_damp(report: &[i32]) -> bool {
        let increasing = report
            .windows(2)
            .map(|w| w[0] < w[1])
            .collect::<Vec<bool>>();

        let decreasing = report
            .windows(2)
            .map(|w| w[0] > w[1])
            .collect::<Vec<bool>>();

        let diff = report
            .windows(2)
            .map(|w| (w[0] - w[1]).abs() <= 3)
            .collect::<Vec<bool>>();

        let order_bools = if increasing.iter().filter(|&b| *b).count()
            > decreasing.iter().filter(|&b| *b).count()
        {
            increasing
        } else {
            decreasing
        };
        let my_bools = and(&order_bools, &diff);
        let pos = my_bools.iter().position(|&b| !b);
        match pos {
            Some(idx) => {
                fn remove_then_check(report: &[i32], idx: usize) -> bool {
                    let mut x = report.to_vec();
                    x.remove(idx);
                    is_safe(&x)
                }
                remove_then_check(report, idx) || remove_then_check(report, idx + 1)
            }
            None => true,
        }
    }

    let result = data
        .iter()
        .map(|row| is_safe_damp(row))
        .filter(|&b| b)
        .count();
    result
}
pub fn solve() -> (usize, usize) {
    let input_file = BufReader::new(File::open("input/02.txt").unwrap());
    let data = parse_data(input_file);
    let p1 = part1(&data);
    let p2 = part2(&data);
    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_part1() {
        let input_file = BufReader::new(TEST.as_bytes());
        let data = parse_data(input_file);
        assert_eq!(2, part1(&data));
    }

    #[test]
    fn test_part2() {
        let input_file = BufReader::new(TEST.as_bytes());
        let data = parse_data(input_file);
        assert_eq!(4, part2(&data));
    }

    #[test]
    fn test_solution() {
        assert_eq!((402, 455), solve());
    }
}
