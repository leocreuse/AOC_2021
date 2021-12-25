//  AOC Day 12

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::collections::HashSet;
// use std::collections::HashMap;

#[derive(Debug)]
struct Cave {
    name: String,
    small: bool,
    neighbors: Vec<usize>
}

fn parse_input (filename: &str) -> io::Result<Vec<(String,String)>> {
    let mut res: Vec<(String, String)> = Vec::new();
    let input = Path::new(filename);
    let file = File::open(input)?;
    let mut lines = io::BufReader::new(file).lines();
    while let Some(Ok(line)) = lines.next() {
        let (start, end) = line.split_once('-').expect("Line not separated??");
        res.push((String::from(start), String::from(end)));
    }
    Ok(res)
}

fn build_graph (arcs: Vec<(String, String)>) -> Vec<Cave>{
    let mut caves: Vec<Cave> = Vec::new();
    for (start, end) in arcs {
        let start_cave_idx = caves.iter().position(|cv| cv.name == start);
        let end_cave_idx = caves.iter().position(|cv| cv.name == end);
        let start_idx = match start_cave_idx {
            Some(idx) => idx,
            None => {
                caves.push (Cave {small: start.chars().all(|c| matches!(c, 'a'..='z')), name: start, neighbors: Vec::new()});
                caves.len() - 1
            }
        };
        let end_idx = match end_cave_idx {
            Some(idx) => idx,
            None => {
                caves.push (Cave {small: end.chars().all(|c| matches!(c, 'a'..='z')), name: end, neighbors: Vec::new()});
                caves.len() - 1
            }
        };
        caves[start_idx].neighbors.push(end_idx);
        caves[end_idx].neighbors.push(start_idx);

    }
    caves
}

fn count_paths (graph: &mut Vec<Cave>, current_idx:usize, end_idx: usize, seen: &Vec<bool>, double_used: bool, root: usize) -> i32{
    let mut res: i32 = 0;
    if current_idx == end_idx {
        println!("found the end!");
        res = 1;
    }
    else if !seen[current_idx] {
        println!("visiting {}", current_idx);
        let mut new_seen = seen.clone();
        new_seen[current_idx] = graph[current_idx].small;
        for child_idx in 0 .. graph [current_idx].neighbors.len() {
            res += count_paths(graph, graph [current_idx].neighbors[child_idx], end_idx, &new_seen, double_used, root);
        }
    } else if current_idx != root && !double_used {
        println!("visiting {} (double burned)", current_idx);
        for child_idx in 0 .. graph [current_idx].neighbors.len() {
            res += count_paths(graph, graph [current_idx].neighbors[child_idx], end_idx, &seen, true, root);
        }
    }
    res
}

fn main() -> io::Result<()> {
    let arcs = parse_input("./input.txt")?;
    let mut graph = build_graph(arcs);
    for (idx, cv) in graph.iter().enumerate() {
        println!{"{}: {}", cv.name, idx};
    }
    let start_idx = graph.iter().position(|cv| cv.name == "start").expect("did not find start node...");
    let end_idx = graph.iter().position(|cv| cv.name == "end").expect("did not find end node...");
    let num_nodes = graph.len();
    let all_paths = count_paths(&mut graph, start_idx, end_idx,&vec![false; num_nodes], false, start_idx);
    println!("path count: {}", all_paths);

    Ok(())
}
