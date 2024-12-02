use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn parse_data<R: BufRead>(reader: R) -> Result<(Vec<i32>, Vec<i32>)> {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in reader.lines() {
        let line = line?;
        //let row:[i32; 2] = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>().try_into().unwrap();
        let row = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        left.push(row[0]);
        right.push(row[1]);
    }
    left.sort();
    right.sort();
    Ok((left, right))
}

fn part1(left: &Vec<i32>, right: &Vec<i32>) -> Result<usize> {
    let total = left.iter().zip(right.iter()).map(|(&l, &r)| (r - l).abs()).sum::<i32>();
    Ok(total as usize)
}

fn part2(left: &Vec<i32>, right: &Vec<i32>) -> Result<usize> {
    let total = left.iter().map(|&l| right.iter().filter(|&r| *r == l).count() * (l as usize)).sum();
    Ok(total)
}

fn main() -> Result<()> {
    start_day(DAY);
    
    println!("=== Load Data ===");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let (left, right) = time_snippet!(parse_data(input_file))?;
    let (left_test, right_test) = parse_data(BufReader::new(TEST.as_bytes()))?;
    
    println!("\n=== Part 1 ===");
    assert_eq!(11, part1(&left_test, &right_test)?);
    let result = time_snippet!(part1(&left, &right)?);
    println!("Result = {}", result);
    
    println!("\n=== Part 2 ===");
    
    assert_eq!(31, part2(&left_test, &right_test)?);
    
    let result = time_snippet!(part2(&left, &right)?);
    println!("Result = {}", result);
    Ok(())
}
