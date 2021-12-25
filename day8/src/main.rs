//  AOC Day 8

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;

fn parse_input (filename: &str) -> io::Result<Vec<(Vec<String>, Vec<String>)>> {
    let mut res:Vec<(Vec<String>, Vec<String>)> = Vec::new();
    let input = Path::new(filename);
    let file = File::open(input)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(line_str) = line {
            let mut pattern_vec = Vec::new();
            let mut output_vec = Vec::new();
            let (patterns, output_values) = line_str.split_once('|').expect("Did not find '|' char");
            for pat in patterns.split_whitespace() {
                pattern_vec.push(String::from(pat));
            }
            for output in output_values.split_whitespace() {
                output_vec.push(String::from (output));
            }
            res.push((pattern_vec, output_vec));
        }
    }
    Ok(res)
}

//  Segment codes:
//  10^010^010^0
//  10^1    10^2
//  10^310^310^3
//  10^4    10^5
//  10^310^610^6

fn decode_segments (patterns: &Vec<String>) -> HashMap<char, u64>{
    let mut res = HashMap::new();
    let mut one: HashSet<char> = HashSet::new();
    let mut seven: HashSet<char> = HashSet::new();
    let mut four: HashSet<char> = HashSet::new();
    let mut seg5: Vec<HashSet<char>> = Vec::new();
    let mut seg6: Vec<HashSet<char>> = Vec::new();
    let mut eight: HashSet<char> = HashSet::new();
    for pat in patterns.iter() {
        match pat.len() {
            2 => one = pat.chars().collect(),
            3 => seven = pat.chars().collect(),
            4 => four = pat.chars().collect(),
            5 => seg5.push(pat.chars().collect()),
            6 => seg6.push(pat.chars().collect()),
            7 => eight = pat.chars().collect(), 
            _ => panic!("Unexpected number of segments")

        };
    }

    let char5 = *(&(&(&seg6[0] & &seg6[1]) & &seg6 [2]) & &one).iter().next().expect("intersection is empty!!");
    res.insert(char5, 100000); 
    one.remove(&char5);
    seven.remove(&char5);
    four.remove(&char5);
    seg5[0].remove(&char5);
    seg5[1].remove(&char5);
    seg5[2].remove(&char5);
    seg6[0].remove(&char5);
    seg6[1].remove(&char5);
    seg6[2].remove(&char5);
    eight.remove (&char5);
    let char2 = *one.iter().next().expect("one is empty!!");
    res.insert(char2, 100); 
    one.remove(&char2);
    seven.remove(&char2);
    four.remove(&char2);
    seg5[0].remove(&char2);
    seg5[1].remove(&char2);
    seg5[2].remove(&char2);
    seg6[0].remove(&char2);
    seg6[1].remove(&char2);
    seg6[2].remove(&char2);
    eight.remove (&char2);
    let char0 = *seven.iter().next().expect("seven is empty!!");
    res.insert(char0, 1); 
    seven.remove(&char0);
    four.remove(&char0);
    seg5[0].remove(&char0);
    seg5[1].remove(&char0);
    seg5[2].remove(&char0);
    seg6[0].remove(&char0);
    seg6[1].remove(&char0);
    seg6[2].remove(&char0);
    eight.remove (&char0);
    let char3 = *(&(&(&seg5[0] & &seg5[1]) & &seg5 [2]) & &four).iter().next().expect("intersection is empty!!");
    res.insert(char3, 1000);
    four.remove(&char3);
    seg5[0].remove(&char3);
    seg5[1].remove(&char3);
    seg5[2].remove(&char3);
    seg6[0].remove(&char3);
    seg6[1].remove(&char3);
    seg6[2].remove(&char3);
    eight.remove (&char3);
    let char1 = *four.iter().next().expect("four is empty!!");
    res.insert(char1, 10);
    four.remove(&char1);
    seg5[0].remove(&char1);
    seg5[1].remove(&char1);
    seg5[2].remove(&char1);
    seg6[0].remove(&char1);
    seg6[1].remove(&char1);
    seg6[2].remove(&char1);
    eight.remove (&char1);
    let char6 = *(&(&seg5[0] & &seg5[1]) & &seg5 [2]).iter().next().expect("intersection is empty");
    res.insert(char6, 1000000);
    seg5[0].remove(&char6);
    seg5[1].remove(&char6);
    seg5[2].remove(&char6);
    seg6[0].remove(&char6);
    seg6[1].remove(&char6);
    seg6[2].remove(&char6);
    eight.remove (&char6);
    let char4 = *eight.iter().next().expect("six is empty!!");
    res.insert(char4, 10000);


    res
}

fn convert_outputs(outputs: &Vec<String>, mapping: HashMap<char, u64>) -> u64{
    let mut res = 0;
    for (pos, output) in outputs.iter().enumerate() {
        let output_mapped = output.chars().fold(0u64, |acc:u64, chr: char| -> u64 {acc + mapping.get(&chr).expect("mapp incomplete")});
        println!("output_mapped: {}", output_mapped);
        res += 10u64.pow((3-pos) as u32) * match output_mapped {
            100100 => 1,
            1011101 => 2,
            1101101 =>3,
            101110 => 4,
            1101011 => 5,
            1111011 => 6,
            100101 => 7,
            1111111 => 8,
            1101111 => 9,
            1110111 => 0,
            _ => panic!("weird mapping")
        }
    }
    res
}

fn main() -> io::Result<()> {
    let parsed_input= parse_input("./input.txt")?;
    //  println!("{:?}", parsed_input);
    let num_of_1478 = parsed_input.iter().fold(0, |acc: i32, entry | -> i32 {acc + entry.1.iter().fold(0, |acc2, output| {let len = output.len(); acc2 + if (len == 2) || (len == 3) || (len == 4) || (len == 7) {1} else {0}})});
    println!("The number of 1, 4, 7 and 8 is {}", num_of_1478);
    let final_count = parsed_input.iter().fold(0u64, |acc:u64, (patterns, outputs): &(Vec<String>, Vec<String>)| -> u64 {
        acc + convert_outputs(outputs, decode_segments(patterns))
    });
    println!("output count: {}", final_count);
    Ok(())
}
