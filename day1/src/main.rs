// AOC day 1

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn readlines(pathname : &str) -> io::Result<Vec<String>> {
    let input = Path::new(pathname);
    let file = File::open(input)?;
    let mut res : Vec<String> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        res.push(line?);
    }
    Ok(res)
}

fn main() -> io::Result<()> {
    let mut first : bool = true;
    let mut previous_single: i32 = 0;
    let mut previous_triple: i32 = 0;
    let mut count_single = 0;
    let mut count_triple = 0;

    // let input = Path::new("./input.txt");
    // let file = File::open(input)?;
    let lines = readlines("./input.txt")?;

    for index in 0..lines.len() - 2 {

        let current_single: i32 = lines[index].parse::<i32>().unwrap();
        let current_triple: i32 = lines[index].parse::<i32>().unwrap() + lines[index+1].parse::<i32>().unwrap() + lines[index+2].parse::<i32>().unwrap();
        if !first && current_single > previous_single {
            count_single += 1;
        }
        if !first && current_triple > previous_triple {
            count_triple +=1;
        }
        first = false;
        previous_single = current_single;
        previous_triple = current_triple;
    }
    if lines[lines.len() - 2].parse::<i32>().unwrap() > lines[lines.len() - 3]  .parse::<i32>().unwrap() {
        count_single += 1;
    }
    if lines[lines.len() - 1].parse::<i32>().unwrap() > lines[lines.len() - 2].parse::<i32>().unwrap() {
        count_single += 1;
    }
    
    println!("There are {} measurements that are larger than the previous one.", count_single);
    println!("There are {} measurements that are larger than the previous one using a sliding window.", count_triple);

    Ok(())
}
