use std::collections::HashSet;
use std::fs;

pub fn solve() {
    println!("Day 5 Part 1 Solution: {}", part1());
    println!("Day 5 Part 2 Solution: {}", part2());
}

fn part1() -> u64 {
    let input = fs::read_to_string("./src/day5/input.txt").unwrap();
    let mut result = 0;
    let lines = input.lines();
    let mut ranges: Vec<Vec<u64>> = Vec::new();
    let mut has_all_ranges = false;
    for line in lines {
        if line.is_empty() {
            has_all_ranges = true;
            continue;
        }
        if has_all_ranges {
            let mut spoiled = true;
            'spoiledCheck: for range in &ranges {
                let value = line.parse::<u64>().unwrap();
                if value >= range[0] && value <= range[1] {
                    spoiled = false;
                    break 'spoiledCheck;
                }
            }
            if !spoiled {
                result += 1;
            }
        } else {
            let mut range = line
                .split("-")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            ranges.push(range);
        }
    }

    result
}

fn part2() -> u64 {
    let input = fs::read_to_string("./src/day5/input.txt").unwrap();
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let parts: Vec<u64> = line.split("-").map(|x| x.parse().unwrap()).collect();
        ranges.push((parts[0], parts[1]));
    }

    ranges.sort_by_key(|r| r.0);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    for range in ranges {
        if let Some(last) = merged.last_mut() {
            if range.0 <= last.1 + 1 { 
                last.1 = last.1.max(range.1);
            } else {
                merged.push(range);
            }
        } else {
            merged.push(range);
        }
    }

    merged.iter().map(|(start, end)| end - start + 1).sum()
}