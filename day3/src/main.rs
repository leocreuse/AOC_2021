//  AOC Day 3

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

fn part1() -> io::Result<()>{
    let input = Path::new("./input.txt");
    let file = File::open(input)?;
    let mut lines = io::BufReader::new(file).lines();
    let first_line = lines.next().unwrap()?;
    let mut col_sums: Vec<i32> = Vec::new();
    for bit in first_line.chars() {
        match bit {
            '0' => col_sums.push(-1),
            '1' => col_sums.push(1),
            _ => ()
        }
    }
    for line in lines {
        for (idx, bit) in line?.chars().enumerate(){
            col_sums[idx] += match bit {
                '0' => -1,
                '1' => 1,
                _ => 0
            }
        }
    }
    let mut result: usize = 0;
    let mask: usize = 2usize.pow(first_line.len() as u32) - 1;
    col_sums.reverse();
    for bit_num in 0 .. col_sums.len() {
        result += if col_sums[bit_num] > 0 {
            2usize.pow(bit_num as u32)
        } else { 0 as usize };
    }
    println!("Gamma rate: {}", result);
    println!("Epsilon rate: {}", !result & mask);
    println!("Gama x Epsilon: {}", result * (!result & mask));

    Ok(())
}

fn bit_count (lines: &[Vec<char>], index: usize) -> (u32, u32) {
    let mut acc_1: u32 = 0;
    let mut acc_0: u32 = 0;
    for line in lines {
        match line[index] {
            '1' => acc_1 +=1,
            '0' => acc_0 +=1,
            _ => () 
        };
    }
    (acc_1,acc_0)
}


fn part2() -> io::Result<()> {

    //  first the oxy rate
    let mut lines_oxy: Vec<Vec<char>> = Vec::new();
    for line in readlines("./input.txt")? {
        lines_oxy.push(line.chars().collect());
    }
    let mut current_bit: usize =0;

    while lines_oxy.len() > 1 {
        let (num_1, num_0) = bit_count(lines_oxy.as_slice(), current_bit);
        let filter_char = if num_1 >= num_0 {'1'} else {'0'};
        let mut current_line: usize = 0;
        while current_line < lines_oxy.len() {
            if lines_oxy[current_line][current_bit] != filter_char {
                lines_oxy.swap_remove(current_line);
            } else {
                current_line += 1;
            }
        }
        current_bit += 1;
    }

    //  then the co2 rate
    let mut lines_co2: Vec<Vec<char>> = Vec::new();
    for line in readlines("./input.txt")? {
        lines_co2.push(line.chars().collect());
    }

    let mut oxy_rate: i32 = 0;
    let bit_length: usize = (&lines_oxy[0]).len();

    for bit in 0..bit_length{
        if (&lines_oxy[0])[bit] == '1' {
            oxy_rate += 2i32.pow((bit_length - bit - 1) as u32);
        }
    }
    
    current_bit = 0;

    while lines_co2.len() > 1 {
        let (num_1, num_0) = bit_count(lines_co2.as_slice(), current_bit);
        let filter_char = if num_1 < num_0 {'1'} else {'0'};
        let mut current_line: usize = 0;
        while current_line < lines_co2.len() {
            if lines_co2[current_line][current_bit] != filter_char {
                lines_co2.swap_remove(current_line);
            } else {
                current_line += 1;
            }
        }
        current_bit += 1;
    }

    let mut co2_rate: i32 = 0;

    for bit in 0..(&lines_co2[0]).len(){
        if (&lines_co2[0])[bit] == '1' {
            co2_rate += 2i32.pow((bit_length - bit - 1) as u32);
        }
    }
    println!("Oxygen rate: {}", oxy_rate);
    println!("CO2 rate: {}", co2_rate);
    println!("answer: {}", oxy_rate * co2_rate);


    Ok(())
}

fn main() -> io::Result<()> {
    part1()?;
    part2()
}
