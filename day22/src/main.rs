//  AOC Day 15

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
// use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive (Clone, Copy)]
struct Cuboid {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
    zmin: i32,
    zmax: i32,
    on: i32
}
impl Cuboid {
    fn inter (self: & Cuboid, other: & Cuboid) -> Option<Cuboid> {
        let xmin = i32::max(self.xmin, other.xmin);
        let xmax = i32::min(self.xmax, other.xmax);
        let ymin = i32::max(self.ymin, other.ymin);
        let ymax = i32::min(self.ymax, other.ymax);
        let zmin = i32::max(self.zmin, other.zmin);
        let zmax = i32::min(self.zmax, other.zmax);
        if xmin > xmax || ymin > ymax || zmin > zmax {
            None
        } else {
            Some (Cuboid {xmin, xmax, ymin, ymax, zmin, zmax, on: 1})
        }
    }
}

fn parse_input (filename: &str) -> io::Result<(Vec<Cuboid>, i32, i32, i32, i32, i32, i32)> {
    let mut res: Vec<Cuboid> = Vec::new();
    let (mut x_min, mut x_max, mut y_min, mut y_max, mut z_min, mut z_max) = (0,0,0,0,0,0);
    let input = Path::new(filename);
    let file = File::open(input)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let line = line?;
        let line:Vec<&str> = line.split_whitespace().collect();
        let on = if line[0] == "on" {1} else {-1};
        let line:Vec<&str> = line[1].split(',').collect();
        let x_coords = line[0].split_once('=').expect("no =...").1.split('.').collect::<Vec<&str>>();
        let xmin = x_coords[0].parse::<i32>().expect("not a number");
        if xmin < x_min {
            x_min = xmin;
        }
        let xmax = x_coords[2].parse::<i32>().expect("not a number");
        if xmax > x_max {
            x_max = xmax;
        }
        let y_coords = line[1].split_once('=').expect("no =...").1.split('.').collect::<Vec<&str>>();
        let ymin = y_coords[0].parse::<i32>().expect("not a number");
        if ymin < y_min {
            y_min = ymin;
        }
        let ymax = y_coords[2].parse::<i32>().expect("not a number");
        if ymax > y_max {
            y_max = ymax;
        }
        let z_coords = line[2].split_once('=').expect("no =...").1.split('.').collect::<Vec<&str>>();
        let zmin = z_coords[0].parse::<i32>().expect("not a number");
        if zmin < z_min {
            z_min = zmin;
        }
        let zmax = z_coords[2].parse::<i32>().expect("not a number");
        if zmax > z_max {
            z_max = zmax;
        }
        res.push(Cuboid{xmin, xmax, ymin, ymax, zmin, zmax, on});
    }

    Ok((res, x_min, x_max, y_min, y_max, z_min, z_max))
}

fn apply_cubo (volumes: & mut HashMap<Cuboid, i64>, new: &Cuboid) {
    let mut update: HashMap<Cuboid, i64> = HashMap::new();
    for (old, old_cnt) in volumes.iter() {
        if let Some(inter) = old.inter(new) {
            *update.entry(inter).or_insert(0) -= *old_cnt;
        }
    }
    if new.on == 1 {
        *update.entry(*new).or_insert(0) += 1;
    }
    for (cubo, val) in update {
        *volumes.entry(cubo).or_insert(0) += val;
    }
}

fn volume(cubo: &Cuboid) -> i64 {
    (cubo.xmax - cubo.xmin + 1) as i64 * (cubo.ymax - cubo.ymin + 1) as i64 * (cubo.zmax - cubo.zmin + 1) as i64
}

fn main() -> io::Result<()>{
    let (input, _, _, _, _, _, _) = parse_input("./input.txt")?;
    let mut volumes: HashMap<Cuboid, i64> = HashMap::new();
    for cubo in input {
        apply_cubo(&mut volumes, &cubo);
    }
    //  println!("{:?}", volumes);
    let on_count = volumes.iter().fold(0i64, |acc, (cubo, mult)| acc + (volume(cubo) * *mult));
    println!("Number of on cubos: {}", on_count);
    Ok(())
}
