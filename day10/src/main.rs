//  AOC Day 9

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::collections::HashSet;
// use std::collections::HashMap;

fn parse_input (filename: &str) -> io::Result<Vec<String>> {
    let mut res: Vec<String> = Vec::new();
    let input = Path::new(filename);
    let file = File::open(input)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines{
        res.push(line?);
    }
    Ok(res)
}

fn _print_lines (lines: &Vec<String>) {
    for line in lines {
        println!("{}",line);
    }
}

fn close(opening: char) -> char {
    match opening {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("not a char in the grammar!")

    }
}

fn error_score(chr: char) -> u32 {
    match chr {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}

fn autocomplete_score(chr: char) -> u64 {
    match chr {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0
    }
}

fn split_incomplete_err (lines: Vec<String>) -> (Vec<(String, u64)>, Vec<(String, usize, char, char)>) {
    let mut incomplete_lines: Vec<(String, u64)> = Vec::new();
    let mut error_lines : Vec<(String, usize, char, char)> = Vec::new();

    for line in lines {
        let mut found_error = false;
        let mut error_chr: char = '`';
        let mut error_idx: usize = 0;
        let mut first: bool = true;
        let mut expected_close: Option<char> = None;
        let mut close_stack: Vec<char> = Vec::new();
        // println!("Start new line");
        for (idx, chr) in line.chars().enumerate(){
            // println!("processing {}. Expect {:?}; stack={:?}", chr, expected_close, close_stack);
            if first {
                expected_close = Some(close(chr));
                first = false;
            } else {
                if chr == '(' || chr == '[' || chr == '{' || chr == '<' {
                    if let Some(expect) = expected_close{
                        close_stack.push(expect);
                    };
                    expected_close = Some(close(chr));
                } else {
                    match expected_close {
                        Some(expect)  => {
                            if expect == chr {
                                expected_close = close_stack.pop();
                            } else {
                                found_error = true;
                                error_chr = chr;
                                error_idx = idx;
                                println!("syntax error: expected '{}' but got '{}'", expect, error_chr);
                                break;
                            }
                        },
                        _ => panic!("wired char in stream!")
                    }
                }
            }
        }
        if found_error {
            error_lines.push((line, error_idx, expected_close.expect("did not expect anything and still let me down..."), error_chr));
        } else {
            println!("Found an incomplete line!");
            match expected_close {
                Some(chr) => close_stack.push(chr),
                _ => ()
            };
            close_stack.reverse();
            let mut line_score: u64 =0;
            for chr in close_stack {
                line_score = (line_score * 5) + autocomplete_score(chr);
            }
            incomplete_lines.push((line, line_score));
        }
    }

    (incomplete_lines, error_lines)
}

fn main() -> io::Result<()> {
    let lines = parse_input("./input.txt")?;
    let (mut incompletes,errors) = split_incomplete_err(lines);
    let sum_error = errors.iter().fold(0u32, |acc, (_, _, _, chr)| -> u32 {acc + error_score(*chr)});
    println!("Sum of error scores: {}", sum_error);
    println!("There are {} incomplete lines", incompletes.len());
    incompletes.sort_unstable_by(|l:&(String, u64), r: &(String, u64)| {l.1.cmp(&r.1)});
    println!("Middle error of autocompletion: {}", incompletes[incompletes.len()/2].1);
    Ok(())
}
