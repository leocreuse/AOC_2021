//  AOC Day 15

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::collections::HashSet;
// use std::collections::HashMap;

fn parse_input (filename: &str) -> io::Result<Vec<Vec<u32>>> {
    let mut res: Vec<Vec<u32>> = Vec::new();
    let input = Path::new(filename);
    let file = File::open(input)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(line_str) = line {
            let line_vec : Vec<u32> = line_str.chars().map(|chr| chr.to_digit(10).expect("not a digit!!")).collect();
            res.push(line_vec);

        }
    }
    Ok(res)
}

fn _print_map (map: &Vec<Vec<u32>>) {
    for line in map {
        println!("");
        for risk in line {
            print!("{} ", *risk);
        }
    }
    println!("");
}

fn full_map (map: &Vec<Vec<u32>>, x: &usize, y: &usize) -> u32 {
    let map_dim = map.len();
    let real_x =  x % map_dim;
    let real_y =  y % map_dim;
    let offset = (x / map_dim) + (y / map_dim);
    let map_val = map[real_y][real_x] + (offset as u32);
    if map_val <= 9 {
        map_val
    } else {
        map_val % 9
    }

}

fn choose_next (dist: &Vec<Vec<u32>>, visited: &Vec<Vec<u16>>) -> (usize, usize) {
    let mut current_x: usize = 0;
    let mut current_y: usize = 0;
    let mut current_dist = u32::MAX;
    for y in 0 .. dist.len() {
        for x in 0 .. dist[0].len() {
            if visited[y][x] == 0 && dist[y][x] < current_dist {
                current_dist = dist[y][x];
                current_x = x;
                current_y = y;
            }
        }
    }
    (current_x, current_y)
}

fn _print_dist (dist: &Vec<Vec<u32>>){
    for line in dist {
        println!("");
        for dist in line {
            print!("{} ", dist);
        }
    }
    println!("");
}

fn _print_visit (visit: &Vec<Vec<u16>>){
    for line in visit {
        println!("");
        for dist in line {
            print!("{:>4} ", dist);
        }
    }
    println!("");
}

fn main() -> io::Result<()>{
    let map = parse_input("./input.txt")?;
    //  print_map(&map);
    let map_side = map.len();
    let mut dist = vec![vec![u32::MAX; 5 * map_side]; 5 * map_side];
    println!("{}",dist[0].len());
    let mut visited = vec![vec![0u16; 5 * map_side]; 5 * map_side];
    let num_elem = dist.len() * dist.len();
    dist[0][0] = 0;
    let mut processed_elems = 0;
    while processed_elems < (num_elem as u64) {
        let (x, y) = choose_next(&dist, &visited);
        visited[y][x] = 1;
        //  up
        if y >= 1 {
            let alt = dist[y][x] + full_map(&map, &x,&(y-1));
            if visited[y-1][x] == 0 && alt < dist[y-1][x] {
                dist[y-1][x] = alt;
            }
        }
        // left
        if x >= 1 {
            let alt = dist[y][x] + full_map(&map, &(x-1),&y);
            if visited[y][x-1] == 0 && alt < dist[y][x-1] {
                dist[y][x-1] = alt;
            }
        }
        // down
        if y < dist.len() - 1{
            let alt = dist[y][x] + full_map(&map, &x,&(y+1));
            if visited[y+1][x] == 0 && alt < dist[y+1][x] {
                dist[y+1][x] = alt;
            }
        }
        // right
        if x < dist.len() - 1{
            let alt = dist[y][x] + full_map(&map, &(x+1),&y);
            if visited[y][x+1] == 0 && alt < dist[y][x+1] {
                dist[y][x+1] = alt;
            }
        }
        processed_elems += 1;
        if processed_elems % 1000 == 0 {
            print!("\r                                          ");
            print!("\rprocessed {} cells of {}", processed_elems, num_elem);
        }
    }
    println!("");
    println!("min risk path (1) :{}", dist[map.len()-1][map.len()-1]);
    println!("min risk path (2) :{}", dist[(5*map.len())-1][(5*map.len())-1]);
    // _print_visit(&visited);
    // _print_dist(&dist);
    Ok(())
}
