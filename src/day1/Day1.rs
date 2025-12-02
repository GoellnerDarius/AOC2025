use std::str::Lines;
use std::{cmp, fs};

// pub mod day1;
pub fn solve() {
    println!("Hello, world!");

    println!("Day 1 Part 1 Solution: {}", part1());
    println!("Day 1 Part 2 Solution: {}", part2());
}

fn part1() -> i32 {
    let file = fs::read_to_string("./src/day1/input.txt").unwrap();
    let string = file.lines();
    let mut pos = 50;
    let mut count = 0;
    for s in string {
        if s.starts_with('L') {
            pos -= s.split_at(1).1.parse::<i32>().unwrap();
            pos %= 100;
        } else {
            pos += s.split_at(1).1.parse::<i32>().unwrap();
            pos %= 100;
        }
        if pos == 0 {
            count += 1;
        }
    }
    count
}
fn part2() -> i32 {
    let file = fs::read_to_string("./src/day1/input.txt").unwrap();
    let string = file.lines();
    let mut pos = 50;
    let mut count = 0;

    for s in string {
        if s.starts_with('L') {
            let distance = s.split_at(1).1.parse::<i32>().unwrap();
            if pos == 0 {
                count += distance / 100;
            } else if distance >= pos {
                count += 1 + (distance - pos) / 100;
            }
            pos = ((pos - distance) % 100 + 100) % 100;
        } else {
            let distance = s.split_at(1).1.parse::<i32>().unwrap();
            count += (pos + distance) / 100;
            pos = (pos + distance) % 100;
        }
    }
    count
}
