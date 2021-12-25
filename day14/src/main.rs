//  AOC Day 14

use core::num;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::collections::HashSet;
use std::collections::HashMap;

fn parse_input (filename: &str) -> io::Result<(HashMap<char, u64>, HashMap<(char, char), u64>, HashMap<(char,char),char>)> {
    let mut element_map: HashMap<char, u64> = HashMap::new();
    let mut pairs_map: HashMap<(char, char), u64> = HashMap::new();
    let mut new_element_map: HashMap<(char,char), char> = HashMap::new();
    let input = Path::new(filename);
    let file = File::open(input)?;
    let mut lines = io::BufReader::new(file).lines();

    let starting_poly = lines.next().expect("not starting polymer??")?;
    while let Some(Ok(line)) = lines.next() {
        let mut line_splt = line.split_whitespace();
        let mut pair_it = line_splt.next().expect("no pair???").chars();
        let pair = (pair_it.next().expect("no pair first char??"), pair_it.next().expect("no pair second char??"));
        line_splt.next();
        let new_elem = line_splt.next().expect("no new elem??").chars().next().expect("empy string??");
        new_element_map.insert(pair, new_elem);
    }
    let mut poly_it = starting_poly.chars();
    let mut prev_char = poly_it.next().expect("empty starting polymer!!");
    *element_map.entry(prev_char).or_insert(0u64) += 1;

    while let Some(chr) = poly_it.next() {
        *element_map.entry(chr).or_insert(0u64) += 1;
        *pairs_map.entry((prev_char,chr)).or_insert(0u64) += 1;
        prev_char = chr;
    }

    Ok((element_map, pairs_map, new_element_map))
}

fn step (elem_count_map: &mut HashMap<char, u64>, pair_count_map: &mut HashMap<(char, char), u64>, new_elem_map: &HashMap<(char,char),char>){
    let mut new_pairs: HashMap<(char, char), u64> = HashMap::new();
    for (pair, new_elem) in new_elem_map.iter() {
        let num_pairs = match pair_count_map.remove(pair) {None => 0, Some(val) => val};
        *new_pairs.entry((pair.0, *new_elem)).or_insert(0u64) += num_pairs;
        *new_pairs.entry((*new_elem, pair.1)).or_insert(0u64) += num_pairs;
        *elem_count_map.entry(*new_elem).or_insert(0u64) += num_pairs;
    }
    for (pair, count) in new_pairs.drain() {
        pair_count_map.insert(pair, count);
    }
}

fn main() -> io::Result<()>{
    let (mut elem_count_map, mut pair_count_map, new_elem_map) = parse_input("./input.txt")?;
    println!("{:?}", new_elem_map);
    for idx in 1 .. 41 {
        step (&mut elem_count_map, &mut pair_count_map, &new_elem_map);
        println! ("after step {}:", idx);
        let most_common = *elem_count_map.iter().max_by(|l, r| l.1.cmp(r.1)).expect("no max???").1;
        let least_common = *elem_count_map.iter().min_by(|l, r| l.1.cmp(r.1)).expect("no min??").1;
        println!("Min: {}, Max: {}, diff: {}", least_common, most_common, most_common - least_common);

    }
    Ok(())
}
