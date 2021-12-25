//  AOC Day 6

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_input (filename: &str) -> io::Result<[u64; 9]> {
    let mut res = [0u64; 9];
    let input = Path::new(filename);
    let file = File::open(input)?;
    let entries = io::BufReader::new(file).split(',' as u8);
    for entry in entries {
        let entry_str = String::from_utf8(entry?).expect("not an utf8 string");
        println!("{}", &entry_str);
        if entry_str.len() > 0 {
            res[(&entry_str).parse::<usize>().expect("not a number")] += 1;
        }
    }
    Ok(res)
}

fn print_pop (pop: &[u64; 9]) {
    print!("Current populations:{:?}", pop);
    println!(" Total population: {}", pop.iter().fold(0u64, |acc: u64, item:&u64| -> u64 {acc + item}))
}

fn reproduce (pop: &mut [u64;9], days: u64) {
    let mut new_pop = [0u64; 9];
    for day in 0..days {
        println!("processing day {}", day);
        for counter_id in (1usize..9usize).rev() {
            new_pop[counter_id-1] = pop[counter_id];
        }
        new_pop[8] = pop[0];
        new_pop[6] += pop[0];
        for counter_id in 0usize..9usize {
            pop[counter_id] = new_pop[counter_id];
        }
        if day % 10 == 0 {
            print_pop(pop);
        }
    }
}

fn main() -> io::Result<()> {
    let mut population = parse_input("./input.txt")?;
    reproduce(&mut population, 256);
    print_pop(&population);
    Ok(())
}
