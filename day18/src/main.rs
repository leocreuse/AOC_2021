//  AOC Day 18

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone, Copy)]
enum Element {
    Pair(usize, usize),
    Digit (u32)
}

fn parse_element (mut elem: Chars) -> Vec<Element> {
    let mut res : Vec<Element> = Vec::new();
    parse_element_internal(&mut elem, &mut res);
    res
}

fn parse_element_internal (elem: &mut Chars, res: & mut Vec<Element>) {
    let first_char = elem.next().expect("empty element");
    match first_char {
        '0' ..= '9' => res.push(Element::Digit(first_char.to_digit(10).expect("no bueno match dude"))),
        '[' => {
            let insert_index = res.len();
            res.push(Element::Pair (0,0));
            parse_element_internal(elem, res);
            let second_index_insert = res.len();
            assert_eq!(elem.next(), Some(','));
            parse_element_internal(elem, res);
            assert_eq!(elem.next(), Some(']'));
            res[insert_index] = Element::Pair(insert_index + 1, second_index_insert)
        },
        _ => panic!("syntax error: pair is invalid")
    }
}

fn try_split (num : & mut Vec<Element>) -> bool {
    let mut split_index: usize = 0;
    let mut split_found: bool = false;
    let mut split_val : u32 = 0;
    while ! split_found && split_index < num.len() {
        if let Element::Digit(val) = num[split_index] {
            if val >= 10 {
                split_found = true;
                split_val = val;
            } else {
                split_index += 1;
            } 
        } else {
            split_index += 1;
        }
    }
    if !split_found {
        false
    } else {
        num.insert(split_index + 1, Element::Digit(split_val - (split_val / 2)));
        num[split_index] = Element::Digit(split_val/2);
        for idx in 0 .. num.len() {
            match num[idx] {
                Element::Pair(left, right) => {
                    num[idx] = Element::Pair(if left > split_index {left + 2} else {left}, if right > split_index {right + 2} else {right});
                }
                _ => ()
            }
        }
        num.insert(split_index, Element::Pair(split_index+1, split_index+2));
        true
    }
}

fn depth (num: & Vec<Element>, current: usize, target: usize) -> u32 {
    if current == target {
        1
    } else {
        match num[current] {
            Element::Digit(_) => 0,
            Element::Pair(left, right) => {
                let depth_l = depth(num, left, target);
                if depth_l != 0 {
                    1 + depth_l
                } else {
                    let depth_r = depth(num, right, target);
                    if depth_r != 0 {
                        1 + depth_r
                    } else {
                        0
                    }
                }
                
            }
        }
    }
}

fn try_explode (num: & mut Vec<Element>) -> bool {
    //  println!("before exploding: {:?}", num);
    let mut current_idx: usize = 0;
    let mut explode_idx: usize = 0;
    let mut right_add: u32 = 0;
    let mut left_idx: Option<usize> = None;
    let mut left_val: u32 = 0;
    let mut explode_found : bool = false;
    let mut explode_done : bool = false;
    while current_idx < num.len()  && !explode_done{
        match (num[current_idx], explode_found) {
            (Element::Digit(val), false) => {left_idx = Some (current_idx); left_val = val},
            (Element::Pair(lft_idx, rght_idx), false) if depth(num, 0, current_idx) == 5 => {
                explode_found = true;
                //  println!("explding at index {}: {:?}", current_idx, num[current_idx]);
                if let Some (idx) = left_idx {
                    if let Element::Digit(lft) = num[lft_idx] {
                        num[idx] = Element::Digit (left_val + lft);
                    }
                }
                if let Element::Digit(rght) = num[rght_idx] {
                    right_add = rght;
                    //  println!("{}",right_add);
                }
                explode_idx = current_idx;
                num.remove(current_idx + 1);
                num.remove(current_idx + 1);
                num[current_idx] = Element::Digit(0);
                //  println!("{:?}", num);
            },
            (Element::Digit(val), true) => {
                num[current_idx] = Element::Digit(val + right_add);
                explode_done = true;
            },
            _ => ()
        }
        current_idx +=1;
    }
    if !explode_found {
        false
    } else {
        for idx in 0 .. num.len() {
            match num[idx] {
                Element::Pair(left, right) => {
                    num[idx] = Element::Pair(if left > explode_idx {left - 2} else {left}, if right > explode_idx {right - 2} else {right});
                }
                _ => ()
            }
        }
        true
    }
}

fn reduce (num: &mut Vec<Element>) {
    let mut last_explode_success = true;
    let mut last_split_success = true;
    while last_explode_success || last_split_success {
        last_explode_success = try_explode(num);
        // if last_explode_success {
        //     println!("explode occured");
        // }
        if ! last_explode_success {
            // println!("trying to split...");
            last_split_success = try_split(num);
            // if last_split_success {
            //     println!("split occured");
            // }
        }
    }
}

fn add_no_reduce (left: &Vec<Element>, right: &Vec<Element>) -> Vec<Element> {
    let mut res = vec![Element::Pair(1,left.len() + 1)];
    res.extend(left.iter());
    for idx in 1 .. res.len() {
        match res[idx] {
            Element::Digit(_) => (),
            Element::Pair(lft, rght) => res[idx] = Element::Pair(lft + 1, rght + 1)
        }
    }
    let new_first = res.len();
    res.extend(right.iter());
    for idx in new_first .. res.len() {
        match res[idx] {
            Element::Digit(_) => (),
            Element::Pair(lft, rght) => res[idx] = Element::Pair(lft + new_first, rght + new_first)
        }
    }
    res

}

fn add (left: &Vec<Element>, right: &Vec<Element>) -> Vec<Element> {
    let mut res = add_no_reduce (left, right);
    reduce(&mut res);
    res
}

fn magnitude (num: &Vec<Element>, root: usize) -> u32 {
    match num[root] {
        Element::Digit(val) => val,
        Element::Pair(left, right) => 3 * magnitude(num, left) + 2 * magnitude(num, right)
    }
}

fn parse_input (filename: &str) -> io::Result<Vec<Vec<Element>>> {
    let mut res: Vec<Vec<Element>> = Vec::new();
    let input = Path::new(filename);
    let file = File::open(input)?;
    let mut lines = io::BufReader::new(file).lines();
    while let Some (Ok(line)) = lines.next() {
        res.push(parse_element(line.chars()));
    }
    Ok (res)
}

fn pp_element (elem: &Vec<Element>, root: usize) {
    match elem[root] {
        Element::Digit(val) => print!("{}", val),
        Element::Pair(left, right) => {
            print!("[");
            pp_element(elem, left);
            print!(",");
            pp_element(elem,right);
            print!("]")
        }
    }
}

fn main() -> io::Result<()> {
    let numbers = parse_input("./input.txt")?;
    let mut res: Vec<Element> = add(&numbers[0], &numbers[1]);
    for idx in 2 .. numbers.len() {
        res = add(&res, &numbers[idx]);
    }
    pp_element(&res, 0);
    println!("\nmagnitude: {}", magnitude(&res, 0));

    let mut current_max = 0u32;
    for left in 0 .. numbers.len() {
        for right in 0 .. numbers.len() {
            if left != right {
                let res = add (&numbers[left], &numbers[right]);
                let mag = magnitude (&res, 0);
                if mag > current_max {
                    current_max = mag;
                }
            }
        }
    }
    println!("max pair sum magnitude: {}", current_max);

    Ok(())
}
