use adv_code_2024::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

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

fn main() {
    start_day(DAY);

    println!("=== Load Data ===");
    let input_file = BufReader::new(File::open(INPUT_FILE).unwrap());
    let data = time_snippet!(parse_data(input_file));
    let data_test = parse_data(BufReader::new(TEST.as_bytes()));

    println!("\n=== Part 1 ===");
    assert_eq!(2, part1(&data_test));
    let result = time_snippet!(part1(&data));
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");

    assert_eq!(4, part2(&data_test));

    let result = time_snippet!(part2(&data));
    println!("Result = {}", result);
}
