use std::fs;

pub fn solve() {
    println!("Day 6 Part 1 Solution: {}", part1());
    println!("Day 6 Part 2 Solution: {}", part2());
}

fn part1() -> u64 {
    let input = fs::read_to_string("./src/day6/input.txt").unwrap();

    let mut problems = input
        .lines()
        .map(|x| {
            x.split(" ")
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();
    let mut total = 0;
    for i in 0..problems[0].len() {
        let sign = problems.last().unwrap()[i];
        let mut sub_total: u64 = problems[0][i].parse().unwrap();
        for j in 1..problems.len() - 1 {
            if sign == "*" {
                sub_total *= problems[j][i].parse::<u64>().unwrap();
            } else if sign == "+" {
                sub_total += problems[j][i].parse::<u64>().unwrap();
            }
        }
        total += sub_total;

        println!("{sub_total}")
    }

    total
}
fn part2() -> u64 {
    let input = fs::read_to_string("./src/day6/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let max_width = lines.iter().map(|l| l.len()).max().unwrap();

    let mut total = 0;
    let mut numbers: Vec<u64> = Vec::new();
    let mut op = '+';

    let char_at = |row: usize, col| -> char {
        lines[row].chars().nth(col).unwrap_or(' ')
    };

    let apply = |op, nums: &Vec<u64>| -> u64 {
        if op == '*' { nums.iter().product() } else { nums.iter().sum() }
    };

    for col in (0..max_width).rev() {
        let digits: String = (0..lines.len() - 1)
            .map(|row| char_at(row, col))
            .filter(|c| c.is_ascii_digit())
            .collect();

        let op_char = char_at(lines.len() - 1, col);
        if op_char == '+' || op_char == '*' {
            op = op_char;
        }

        if digits.is_empty() {
            if !numbers.is_empty() {
                total += apply(op, &numbers);
                numbers.clear();
            }
        } else {
            numbers.push(digits.parse().unwrap());
        }
    }

    if !numbers.is_empty() {
        total += apply(op, &numbers);
    }

    total
}
