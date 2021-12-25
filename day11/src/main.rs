//  AOC Day 11

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::collections::HashSet;
// use std::collections::HashMap;

struct Octopus {
    energy : i32,
    flashed: bool
}

fn parse_input (filename: &str) -> io::Result<Vec<Vec<Octopus>>> {
    let mut res: Vec<Vec<Octopus>> = Vec::new();
    let input = Path::new(filename);
    let file = File::open(input)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(line_str) = line {
            let mut line_vec : Vec<Octopus> = line_str.chars().map(|chr| Octopus {energy: chr.to_digit(10).unwrap() as i32, flashed: false}).collect();
            line_vec.push(Octopus {energy: -1000i32, flashed: false});
            line_vec.insert(0, Octopus{energy: -1000, flashed: false});
            res.push(line_vec);

        }
    }
    let map_size = res[0].len();
    res.push(vec![-1000i32; map_size].iter().map(|ene| Octopus {energy:*ene, flashed: false}).collect());
    res.insert(0, vec![-1000i32; map_size].iter().map(|ene| Octopus {energy:*ene, flashed: false}).collect());
    Ok(res)
}

fn step(table: &mut Vec<Vec<Octopus>>) -> i32 {
    let mut res: i32 = 0;
    let mut need_processing: Vec<(usize, usize)> = Vec::new();
    for y in 1 .. table.len() -1 {
        for x in 1 .. table[1].len() -1 {
            table [y][x].energy += 1;
            if table [y][x].energy >= 10 {
                need_processing.push((y,x));
            }
        }
    }

    while let Some((y,x)) = need_processing.pop() {
        table[y][x].flashed = true;
        for (n_y, n_x) in [(y-1, x-1),(y, x-1),(y+1, x-1),(y-1, x),(y+1, x),(y-1, x+1),(y, x+1),(y+1, x+1),] {
            table[n_y][n_x].energy +=1;
            if table[n_y][n_x].energy >= 10 && (!table[n_y][n_x].flashed) && !need_processing.contains(&(n_y, n_x)) {
                need_processing.push((n_y, n_x));
            }
        }
        // println!("processing {},{}: {:?}", x, y, need_processing);
    }
    for y in 1 .. table.len() -1 {
        for x in 1 .. table[1].len() -1 {
            if table [y][x].flashed {
                res +=1;
                table[y][x].energy = 0;
                table[y][x].flashed = false;
            }
        }
    }

    res
}

fn _print_map(map: &Vec<Vec<Octopus>>){
    for line in map{
        println!("");
        for oct in line {
            print!("{:>4}, ", oct.energy);
        }
    }
    println!("")
}

fn main() -> io::Result<()> {
    let mut table = parse_input("./input.txt")?;
    let mut flash_count: i32 = 0;
    let mut step_flashes = 0i32;
    let mut iter = 0;
    while step_flashes != 100 {
        if iter % 10 == 0{
            println!("iter {}:", iter);
            _print_map(&table);
        }
        step_flashes = step(&mut table);
        flash_count += step_flashes;
        if step_flashes == 100 {
            println!("Sync falshed at step {}", iter);
        }
        iter +=1;
    }
    println!("got {} flashes!", flash_count);
    Ok(())
}
