//  AOC Day 24

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {Empty, South, East}

use State::{Empty, South, East};

fn parse_input (filename: &str) -> io::Result<Vec<Vec<State>>> {
    let mut res: Vec<Vec<State>> = Vec::new();
    let input = Path::new(filename);
    let file = File::open(input)?;
    let mut lines = io::BufReader::new(file).lines();
    while let Some(Ok(line)) = lines.next(){
        let mut line_vec: Vec<State> = Vec::new();
        for chr in line.chars() {
            match chr {
                '>' => line_vec.push(East),
                'v' => line_vec.push(South),
                '.' => line_vec.push(Empty),
                _ => panic!("unknown map")
            }
        }
        res.push(line_vec);
    }
    Ok(res)
}

fn step_east (map: &mut Vec<Vec<State>>) -> bool {
    let mut res = map.clone();
    let mut moved = false;
    for y in 0 .. map.len() {
        let len = map[y].len();
        for x in 0 .. map[y].len() {
            if map[y][x] == East && map[y][(x+1) % len] == Empty {
                moved = true;
                res[y][(x+1) % len] = East;
                res[y][x] = Empty;
            }
        }
    }
    map.clone_from(&res);
    moved
}

fn step_south (map: &mut Vec<Vec<State>>) -> bool {
    let mut res = map.clone();
    let mut moved = false;
    let len = map.len();
    for y in 0 .. map.len() {
        for x in 0 .. map[y].len() {
            if map[y][x] == South && map[(y+1) % len][x] == Empty {
                moved = true;
                res[(y+1)% len][x] = South;
                res[y][x] = Empty;
            }
        }
    }
    map.clone_from(&res);
    moved
}

fn step (map: &mut Vec<Vec<State>>) ->  bool {
    let mut moved = step_east(map);
    moved |= step_south(map);
    moved
}

fn pp_map (map: & Vec<Vec<State>>) {
    for line in map {
        for pos in line {
            match pos {
                Empty => print!("{}",'.'),
                East => print!("{}",'>'),
                South => print!("{}",'v')
            }
        }
        println!("");
    }
}

fn main() -> io::Result<()> {
    let mut map = parse_input("./input.txt")?;
    pp_map(&map);
    println!("");
    let mut step_count = 1;
    while step(&mut map) {
        pp_map(&map);
        println!("");
        step_count +=1;
    }
    println!("Stationary after {} steps", step_count);
    Ok(())
}
