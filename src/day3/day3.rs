use std::{cmp, fs};

pub fn solve() {
    println!("Day 3 Part 1 Solution: {}", solution(2));
    println!("Day 3 Part 2 Solution: {}", solution(12));
}
fn solution(digit_count: usize) -> u64 {
    let input = fs::read_to_string("./src/day3/input.txt").unwrap();
    let lines = input.lines();
    let mut result = 0;
    for line in lines {
        let mut digits = vec!['0'; digit_count];
        for (i, current_number) in line.chars().enumerate() {
            let start = (digit_count + i).saturating_sub(line.len());
            for digit_index in start..digit_count {
                if current_number > digits[digit_index] {
                    digits[digit_index] = current_number;
                    digits[digit_index + 1..].fill('0');
                    break;
                }
            }
        }
        result += digits.iter().collect::<String>().parse::<u64>().unwrap();
    }
    result
}
