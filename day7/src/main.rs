//  AOC Day 7

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_input (filename: &str) -> io::Result<Vec<i32>> {
    let mut res = Vec::new();
    let input = Path::new(filename);
    let file = File::open(input)?;
    let entries = io::BufReader::new(file).split(',' as u8);
    for entry in entries {
        let entry_str = String::from_utf8(entry?).expect("not an utf8 string");
        if entry_str.len() > 0 {
            res.push((&entry_str).parse::<i32>().expect("not a number"));
        }
    }
    Ok(res)
}

fn fuel_cost (crab_pos: &Vec<i32>, target: i32) -> i32 {
    crab_pos.iter().fold(0, |acc:i32, x :&i32| -> i32 {let n = (target - x).abs(); acc + ((n * (n+1)) /2)})
}

fn main() -> io::Result<()> {
    let crab_pos = parse_input("./input.txt")?;

    let min_pos = *crab_pos.iter().min().unwrap();
    let max_pos = *crab_pos.iter().max().unwrap() + 1;

    let mut current_min_pos = min_pos;
    let mut current_min_cost = fuel_cost(&crab_pos, current_min_pos);

    for pos in min_pos .. max_pos {
        let current_cost = fuel_cost(&crab_pos, pos);
        if current_cost < current_min_cost {
            current_min_pos = pos;
            current_min_cost = current_cost;
        }
    }

    println!("Min cost of {} found at {}", current_min_cost, current_min_pos);

    Ok(())
}
