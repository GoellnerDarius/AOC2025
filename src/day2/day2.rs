use std::fs;
use fancy_regex::Regex;

pub fn solve(){
    println!("Day 2 Part 1 Solution: {}", part1());
    println!("Day 2 Part 2 Solution: {}", part2());
}



fn part1()->i64
{
    let line=fs::read_to_string("./src/day2/input.txt").unwrap();
    let ranges:Vec<&str>=line.split(',').collect();
    let re = Regex::new(r"^(.+)\1$").unwrap();
    let mut count:i64=0;
    for range in &ranges {
        let mut parts = range.split('-');
        let part = (
            parts.next().unwrap().parse::<i64>().unwrap(),
            parts.next().unwrap().parse::<i64>().unwrap()
        );
        for i in part.0..=part.1 {
            if re.is_match(&i.to_string() as &str).unwrap()
            {
                count+=i;
            }
        }


    }

    count
}
fn part2()->i64{

    let line=fs::read_to_string("./src/day2/input.txt").unwrap();
    let ranges:Vec<&str>=line.split(',').collect();
    let re = Regex::new(r"^(.+)\1+$").unwrap();
    let mut count:i64=0;
    for range in &ranges {
        let mut parts = range.split('-');
        let part = (
            parts.next().unwrap().parse::<i64>().unwrap(),
            parts.next().unwrap().parse::<i64>().unwrap()
        );
        for i in part.0..=part.1 {
            if re.is_match(&i.to_string() as &str).unwrap()
            {
                count+=i;
            }
        }


    }

    count
    //30599400849 low
}