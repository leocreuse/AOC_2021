//  AOC Day 4

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_input (filename: &str)-> io::Result<(Vec<i32>, Vec<(Vec<Vec<(i32,bool)>>, bool)>)> {
    let input = Path::new(filename);
    let file = File::open(input)?;
    let mut lines = io::BufReader::new(file).lines();
    let number_draw: Vec<i32>= lines.next().unwrap()?.split(',').map(|str_num: &str| -> i32 {str_num.trim().parse().unwrap()}).collect();
    lines.next();
    let mut grids: Vec<(Vec<Vec<(i32, bool)>>, bool)> = Vec :: new();
    for (idx, line) in lines.enumerate() {
        if (idx % 6) ==0 {
            grids.push((Vec::new(), false));
        }
        if (idx % 6) != 5 {
            grids[idx/6].0.push((&line?).split_whitespace().map(|str_num: &str| -> (i32, bool) {(str_num.trim().parse().unwrap(), false)}).collect());
        }
    }
    Ok((number_draw, grids))
}

fn print_grids(grids: &[(Vec<Vec<(i32,bool)>>, bool)]) {
    for (grid, found) in grids {
        println!("");
        for line in grid {
            println!("{:?}", line);
        }
        println!("found: {}", found);
    }
}

fn grid_won(grid: &[Vec<(i32,bool)>]) -> bool {
    let mut won = false;
    let mut line_idx = 0;
    while !won && line_idx < 5 {
        won = grid[line_idx][0].1 && grid[line_idx][1].1 && grid[line_idx][2].1 && grid[line_idx][3].1 && grid[line_idx][4].1;
        if !won {
            won = grid[0][line_idx].1 && grid[1][line_idx].1 && grid[2][line_idx].1 && grid[3][line_idx].1 && grid[4][line_idx].1;
        }
        line_idx += 1;
    }
    won
}

fn main() -> io::Result<()> {

    let (mut numbers, mut grids) = parse_input("./input.txt")?;
    numbers.reverse();
    let mut last_num: i32 = 0;
    let mut last_grid: usize = 0;
    let mut win_grid_idx: Option<usize> = None;
    let mut next_draw = true;

    let mut remaining_grids = grids.len();
    while remaining_grids > 0 {
        while win_grid_idx == None{
            if next_draw {
                last_num = numbers.pop().unwrap();
            }
            println!("{}", last_num);
            let mut grid_idx:usize = 0;
            while grid_idx < grids.len() && win_grid_idx == None {
                if !grids[grid_idx].1 {
                    let mut line_idx: usize = 0;
                    while line_idx < grids[grid_idx].0.len() {
                        let pos = grids[grid_idx].0[line_idx].iter().position( |&(x,_) : &(i32,bool)| -> bool { x == last_num});
                        match pos {
                            Some (idx) => {println!("match found");
                                                grids[grid_idx].0[line_idx][idx].1 = true; 
                                                win_grid_idx = if grid_won(grids[grid_idx].0.as_slice()) {Some (grid_idx)} else {None}},
                            None => ()
                        };
                        line_idx += 1;
                    }
                }
                grid_idx +=1;
            }
            if win_grid_idx == None {
                next_draw = true;
            }
            else {
                next_draw = false;
            }
        }
        grids[win_grid_idx.unwrap()].1 = true;
        remaining_grids -= 1;
        last_grid = win_grid_idx.unwrap();
        win_grid_idx = None;
        print_grids(grids.as_slice());
        if remaining_grids == grids.len() - 1 {
            let unused_sum: i32 = grids[last_grid].0.iter().fold (0, |acc, line| -> i32 {acc + line.iter().fold(0, |acc, (x, used)| -> i32 {acc + if !*used {x} else {&0}})});
            println!("first result: {}", unused_sum * last_num);
        }
    }
    let unused_sum: i32 = grids[last_grid].0.iter().fold (0, |acc, line| -> i32 {acc + line.iter().fold(0, |acc, (x, used)| -> i32 {acc + if !*used {x} else {&0}})});
    println!("second result: {}", unused_sum * last_num);

    Ok(())
}
