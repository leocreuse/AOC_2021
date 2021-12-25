//  AOC Day 9

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::collections::HashSet;
// use std::collections::HashMap;

fn parse_input (filename: &str) -> io::Result<Vec<Vec<i32>>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let input = Path::new(filename);
    let file = File::open(input)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(line_str) = line {
            let mut line_vec : Vec<i32> = line_str.chars().map(|chr| chr.to_digit(10).unwrap() as i32).collect();
            line_vec.push(9i32);
            line_vec.insert(0, 9);
            res.push(line_vec);

        }
    }
    let map_size = res[0].len();
    res.push(vec![9; map_size]);
    res.insert(0, vec![9; map_size]);
    Ok(res)
}

fn _print_map(map: &Vec<Vec<i32>>){
    for line in map{
        println!("{:?}", line);
    }
}

fn bassin_size (map: &Vec<Vec<i32>>, low_point: (usize,usize)) -> i32{
    let mut explored: Vec<(usize, usize)> = Vec::new();
    let mut unexplored: Vec<(usize, usize)> = Vec::new();
    unexplored.push(low_point);
    while !unexplored.is_empty(){
        let (y,x) = unexplored.pop().unwrap();
        explored.push((y,x));
        for coords in [(y-1,x), (y+1,x), (y,x-1), (y,x+1)]{
            if (map[coords.0][coords.1] != 9) && !explored.contains(&coords) && !unexplored.contains(&coords) {
                unexplored.push(coords);
            }
        }
    }
    println!("bassin size for ({},{}): {}",low_point.1, low_point.0, explored.len());
    explored.len() as i32
}

fn is_low_point (map:  &Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    let current_val = map [y][x];
    current_val < map[y-1][x] && current_val < map[y+1][x] && current_val < map[y][x-1] && current_val < map[y][x+1]
}

fn sum_low_point (map: &Vec<Vec<i32>>) -> (i32, i32){
    let mut res1: i32 = 0;
    let mut bassin_sizes: Vec<i32> = Vec::new();
    for y in 1 .. map.len() - 1{
        for x in 1 .. map[0].len() -1 {
            if is_low_point(map, x, y) {
                res1 += map[y][x] + 1;
                bassin_sizes.push(bassin_size(map, (y,x)));
                
            }
        }
    }
    bassin_sizes.sort();
    let last = bassin_sizes.len() - 1;
    (res1, bassin_sizes[last] * bassin_sizes[last-1] * bassin_sizes[last-2])

}

fn main() -> io::Result<()> {
    let depth_map = parse_input("./input.txt")?;
    //print_map(&depth_map);
    let (res1, res2) = sum_low_point(&depth_map);
    println!("risk factor sum: {}", res1);
    println!("bassin size sum: {}", res2);
    Ok(())
}
