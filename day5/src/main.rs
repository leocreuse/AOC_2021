//  AOC Day 5

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    count: i32
}

fn parse_input (filename :&str) -> io::Result<Vec<Point>>{
    let input = Path::new(filename);
    let file = File::open(input)?;
    let lines = io::BufReader::new(file).lines();
    let mut res: Vec<Point> = Vec::new();
    for (line_num, line) in lines.enumerate() {
        println!("processing entry {}", line_num);
        let line = line?;
        let mut line_it = line.split_whitespace();
        let start_coord: Vec<i32> = line_it.next().expect("empty start point").split(',').map(|num_str: &str| -> i32 {num_str.parse().expect("not a number")}).collect();
        line_it.next();
        let end_coord: Vec<i32> = line_it.next().expect("empty end point").split(',').map(|num_str: &str| -> i32 {num_str.parse().expect("not a number")}).collect();
        if start_coord[0] == end_coord[0] {
            let big_y = if start_coord[1] < end_coord [1] {end_coord [1]} else {start_coord[1]};
            let small_y = if start_coord[1] >= end_coord [1] {end_coord [1]} else {start_coord[1]};
            for i in small_y .. big_y+1 {
                let pos = res.iter().position(|elem : &Point| -> bool {elem.x == start_coord[0] && elem.y == i});
                match pos {
                    None => res.push(Point {x: start_coord[0], y: i, count:1}),
                    Some (idx) => res[idx].count +=1
                };
            }
        }
        else if start_coord [1] == end_coord [1] {
            let big_x = if start_coord[0] < end_coord [0] {end_coord [0]} else {start_coord[0]};
            let small_x = if start_coord[0] >= end_coord [0] {end_coord [0]} else {start_coord[0]};
            for i in small_x .. big_x +1{
                let pos = res.iter().position(|elem : &Point| -> bool {elem.x == i && elem.y == start_coord[1]});
                match pos {
                    None => res.push(Point {x: i, y: start_coord[1], count:1}),
                    Some (idx) => res[idx].count +=1
                };
            }
        }
        else {
            let mut startx : i32 =0;
            let mut starty : i32 =0;
            let mut endx : i32 =0;
            let mut endy : i32 =0;
            let mut y_inc: i32 =1;

            if start_coord[0] < end_coord [0] && start_coord [1] < end_coord [1] {
                startx = start_coord [0];
                starty = start_coord [1];
                endx = end_coord [0];
                endy = end_coord [1];
                y_inc = 1;

            } else if start_coord[0] >= end_coord [0] && start_coord [1] < end_coord [1] {
                startx = end_coord [0];
                starty = end_coord [1];
                endx = start_coord [0];
                endy = start_coord [1];
                y_inc = -1;

            } else if start_coord[0] < end_coord [0] && start_coord [1] >= end_coord [1] {
                startx = start_coord [0];
                starty = start_coord [1];
                endx = end_coord [0];
                endy = end_coord [1];
                y_inc = -1;

            } else if start_coord[0] >= end_coord [0] && start_coord [1] >= end_coord [1] {
                startx = end_coord [0];
                starty = end_coord [1];
                endx = start_coord [0];
                endy = start_coord [1];
                y_inc = 1;
            }
            let mut x = startx;
            let mut y = starty;
            while x != endx + 1 && y != endy + y_inc {
                let pos = res.iter().position(|elem : &Point| -> bool {elem.x == x && elem.y == y});
                match pos {
                    None => res.push(Point {x, y, count:1}),
                    Some (idx) => res[idx].count +=1
                };
                x += 1;
                y += y_inc;
            }
        }
    }

    Ok(res)
}

fn main() -> io:: Result<()> {
    let points = parse_input("./input.txt")?;
    //  println!("{:?}", points);
    let res = points.iter().fold(0, |acc: i32, pts: &Point| -> i32 {acc + (if pts.count > 1 {1} else {0})});
    println!("Result 1: {}", res);
    Ok(())
}
