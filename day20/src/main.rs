//  AOC Day 20

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::collections::HashSet;
// use std::collections::HashMap;

fn parse_input (filename: &str) -> io::Result<(Vec<char>, Vec<Vec<char>>)> {
    let mut res: Vec<Vec<char>> = Vec::new();
    let input = Path::new(filename);
    let file = File::open(input)?;
    let mut lines = io::BufReader::new(file).lines();
    let algo: Vec<char> = lines.next().expect("no algo...")?.chars().map(|chr| if chr == '.' {'0'} else {'1'}).collect();
    let mut first_line = vec!['0'; 50];
    first_line.extend(lines.next().expect("first img line empty...")?.chars().map(|chr| if chr == '.' {'0'} else {'1'}));
    first_line.extend_from_slice(&['0'; 50]);
    let line_len = first_line.len();
    for _ in 0 .. 50 {
        res.push(vec!['0';line_len]);
    }
    res.push(first_line);
    while let Some(Ok(line)) = lines.next() {
        let mut new_line = vec!['0'; 50];
        new_line.extend(line.chars().map(|chr| if chr == '.' {'0'} else {'1'}));
        new_line.extend_from_slice(&['0'; 50]);
        res.push(new_line);
    }
    for _ in 0 .. 52 {
        res.push(vec!['0'; line_len]);
    }
    Ok((algo, res))
}

fn pp_img(img: &Vec<Vec<char>>){
    let line_len = img[0].len();
    print!("+");
    for _ in 0 .. line_len {
        print!("-");
    }
    println!("+");
    for line in img {
        print!("|");
        for chr in line{
            print!("{}", if *chr == '1' {'#'} else {' '});
        }
        println!("|")
    }
    print!("+");
    for _ in 0 .. line_len {
        print!("-");
    }
    println!("+");
}

fn enhance (img: & Vec<Vec<char>>, algo: &Vec<char>) -> Vec<Vec<char>>{
    let mut res: Vec<Vec<char>> = img.clone();
    for y in 0 .. img.len() {
        for x in 0 .. img[0].len() {
            if  y == 0 || y == img.len()-1 || x == 0 || x == img[0].len()-1{
                let code: usize = if img[y][x] == '1' {511} else {0};
                res[y][x] = algo[code];
            } else {
                let mut code_str: String = String::new();
                code_str.extend(img[y-1][x-1 .. x+2].iter());
                code_str.extend(img[y][x-1 .. x+2].iter());
                code_str.extend(img[y+1][x-1 .. x+2].iter());
                // println!("{}", code_str);
                let code: usize = usize::from_str_radix(&code_str, 2).expect("bin values not ok..");
                res[y][x] = algo[code];
            }
        }
    }
    res
}

fn main() -> io::Result<()> {
    let (algo, mut img) = parse_input("./input.txt")?;
    pp_img(&img);
    img = enhance(&img, &algo);
    img = enhance(&img, &algo);
    pp_img(&img);
    println!("number of bright pixels after two iters: {}", img.iter().fold(0u32, |acc, line| acc + line.iter().fold(0u32, |acc_ln, chr| acc_ln + if *chr == '1' {1} else {0})));
    for _ in 2 .. 50 {
        img = enhance (&img, &algo);
    }
    pp_img(&img);
    println!("number of bright pixels after 50 iters: {}", img.iter().fold(0u32, |acc, line| acc + line.iter().fold(0u32, |acc_ln, chr| acc_ln + if *chr == '1' {1} else {0})));

    Ok(())
}
