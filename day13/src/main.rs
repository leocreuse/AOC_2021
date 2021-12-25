//  AOC Day 13

use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
// use std::collections::HashMap;

#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

enum FoldDir {
    Vert,
    Hori
}

fn parse_inputs (filename_dots: &str, filename_folds: & str) -> io::Result<(HashSet<Point>, Vec<(FoldDir, i32)>)> {
    let mut res_dots: HashSet<Point> = HashSet::new();
    let input_dots = Path::new(filename_dots);
    let file_dots = File::open(input_dots)?;
    let mut lines_dots = io::BufReader::new(file_dots).lines();
    while let Some(Ok(line)) = lines_dots.next() {
        let (x_str, y_str) = line.split_once(',').expect("Line not separated??");
        res_dots.insert(Point {x: x_str.parse().expect("not a numer"), y: y_str.parse().expect("not a number")});
    }
    let mut res_folds: Vec<(FoldDir, i32)> = Vec::new();
    let input_folds = Path::new(filename_folds);
    let file_folds = File::open(input_folds)?;
    let mut line_folds = io::BufReader::new(file_folds).lines();
    while let Some(Ok(line)) = line_folds.next() {
        let line = line.split_whitespace().collect::<Vec<&str>>() [2];
        let (dir, index) = line.split_once('=').expect("no = sign in fold instr");
        let dir = match dir.chars().next().expect("empty fold dir!") {'x' => FoldDir::Vert, 'y' => FoldDir::Hori, _ => panic!("unexpected fold dir")};
        res_folds.push((dir, index.parse::<i32>().expect("not a number")));
    }
    Ok((res_dots, res_folds))
}

fn apply_fold (points: &mut HashSet<Point>, fold: &(FoldDir, i32)) {
    let mut new_points: HashSet<Point> = HashSet::new();
    match &fold.0 {
        FoldDir::Vert => {
            for point in points.iter() {
                if point.x > fold.1 {
                    let new_x = (2 * fold.1) - point.x;
                    new_points.insert(Point{x: new_x, y: point.y});
                }
            }
            points.retain(|pt| pt.x < fold.1);
        }
        FoldDir::Hori => {
            for point in points.iter() {
                if point.y > fold.1 {
                    let new_y = (2 * fold.1) - point.y;
                    new_points.insert(Point{x: point.x, y: new_y});
                }
            }
            points.retain(|pt| pt.y < fold.1);
        }
    }
    for point in new_points.iter() {
        points.insert(*point);
    }

}

fn print_bitmap (points: &HashSet<Point>) {
    let mut x_max = 0;
    let mut y_max = 0;
    for point in points.iter() {
        if point.x > x_max {
            x_max = point.x;
        }
        if point.y > y_max {
            y_max = point.y;
        }
    }
    for y in 0 .. y_max + 1 {
        println!("");
        for x in 0 .. x_max + 1 {
            if points.contains(&Point {x, y}) {
                print!("*");
            } else {
                print!(" ");
            }
        }
    }
    println!("");
}

fn main() -> io::Result<()>{
    let (mut point_set, fold_list) = parse_inputs("input_dots.txt", "input_folds.txt")?;
    println!("Starting with {} points.", point_set.len());
    for (idx,fold) in fold_list.iter().enumerate() {
        apply_fold(&mut point_set, &fold);
        println!("After fold #{}, got {} points.", idx, point_set.len());
        print_bitmap(&point_set);
    }

    Ok(())
}
