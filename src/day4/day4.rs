use std::fs;

pub fn solve() {
    println!("Day 4 Part 1 Solution: {}", part1());
    println!("Day 4 Part 2 Solution: {}", part2());
}

fn part1() -> u64 {
    let input = fs::read_to_string("./src/day4/input.txt").unwrap();
    //  println!("{input}");
    let lines: Vec<Vec<u8>> = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.as_bytes().to_vec())
        .collect();    let mut result: u64 = 0;
    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            if lines[y][x] == b'@' && collectable(x, y, &lines) {
                result += 1;
            }
        }
    }
    result
    //1505
}

fn collectable(x: usize, y: usize, input: &Vec<Vec<u8>>) -> bool {
    let mut non_free_count = 0;
    let y_start = y.saturating_sub(1);
    let y_end = y.saturating_add(1).min(input.len() - 1);
    let x_start = x.saturating_sub(1);
    let x_end = x.saturating_add(1).min(input[0].len() - 1);
    for yl in y_start..=y_end {
        for xl in x_start..=x_end {
            if xl == x && yl == y {
                continue;
            }
            if input[yl][xl] == b'@' {
                non_free_count += 1;
            }
        }
    }
    non_free_count < 4
}

fn part2() -> usize {
    let input = fs::read_to_string("./src/day4/input.txt").unwrap();
    let mut lines:  Vec<Vec<u8>> = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.as_bytes().to_vec())
        .collect();
solve_part2(&mut lines, 0)
}
fn solve_part2(lines:&mut Vec<Vec<u8>>,count:usize) -> usize {
    let mut result: usize = count;
    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            if lines[y][x] == b'@' && collectable(x, y, lines) {
                result += 1;
                lines[y][x]=b'.';
            }
        }
    }
    if count!=result{
      result=  solve_part2(lines,result);
    }
    result
}
