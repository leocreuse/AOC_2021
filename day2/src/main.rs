//  AOC Day 2

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

    let mut aimdepth: i32 = 0;
    let mut depth2: i32 = 0;
    let mut pos: i32 = 0;
    let lines = readlines("./input.txt")?;
    for line in lines {
        let mut splt = line.split_whitespace();
        let command = splt.next().unwrap();
        let val: i32 = splt.next().unwrap().parse().unwrap();
        match command {
            "down" => aimdepth += val,
            "up" => aimdepth -= val,
            "forward" => {pos += val; depth2 += aimdepth * val},
            _ =>()
        };
    }
    println!("Final Position:{}", pos);
    println!("Final depth1: {}", aimdepth);
    println!("Final value1: {}", pos * aimdepth);
    println!("Final depth2: {}", depth2);
    println!("Final value2 {}", pos * depth2);
    Ok(())
}
