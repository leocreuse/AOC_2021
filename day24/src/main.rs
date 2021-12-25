//  AOC Day 24

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::collections::HashSet;
// use std::collections::HashMap;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operand {
    Reg (usize),
    Lit (i64)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instr {
    Inp (usize),
    Add (usize, Operand),
    Mul (usize, Operand),
    Div (usize, Operand),
    Mod (usize, Operand),
    Eql (usize, Operand)
}

fn parse_input (filename: &str) -> io::Result<Vec<Instr>> {
    let mut res: Vec<Instr> = Vec::new();
    let input = Path::new(filename);
    let file = File::open(input)?;
    let mut lines = io::BufReader::new(file).lines();
    while let Some(Ok(line)) = lines.next(){
        let mut splt = line.split_whitespace();
        let insn_str = splt.next().expect("emtpy insn");
        let op1 = (splt.next().expect("empty_first_op").chars().next().expect("empty first op") as u8 - 'w' as u8) as usize;
        let op2: Option<Operand> = splt.next().map(
            |op| -> Operand {
                match op.chars().next().expect("empty operand") {
                    chr @'w'..='z' => Operand::Reg((chr as u8 - 'w' as u8) as usize),
                    _ => Operand::Lit(op.parse::<i64>().expect("not a valid number"))
                }
            });
        let insn = match (insn_str, op2) {
            ("inp", _) => Instr::Inp(op1),
            ("add", Some(op)) => Instr::Add(op1, op),
            ("mul", Some(op)) => Instr::Mul(op1, op),
            ("div", Some(op)) => Instr::Div(op1, op),
            ("mod", Some(op)) => Instr::Mod(op1, op),
            ("eql", Some(op)) => Instr::Eql(op1, op),
            _ => panic!("unknown instruction")
        };
        res.push(insn);
    }
    Ok(res)
}

fn pp_prog (prog: &Vec<Instr>) {
    for insn in prog {
        println!("{:?}", insn);
    }
}

#[inline(always)]
fn decode_op2 (op2: &Operand, regs: [i64;4]) -> i64 {
    match op2 {
        Operand::Lit(val) => *val,
        Operand::Reg(reg_src) => regs[*reg_src]
    }
}

fn run (prog: &Vec<Instr>, inputs: &Vec<i64>) -> [i64;4] {
    let mut regs = [0i64; 4];
    let mut current_input = 0usize;
    for insn in prog {
        match insn {
            Instr::Inp(reg) => {regs[*reg] = inputs[current_input]; current_input+=1},
            Instr::Add(reg, op2) => {
                let op2 = decode_op2(op2, regs);
                regs[*reg] += op2
            },
            Instr::Mul(reg, op2) => {
                let op2 = decode_op2(op2, regs);
                regs[*reg] *= op2
            },
            Instr::Div(reg, op2) => {
                let op2 = decode_op2(op2, regs);
                regs[*reg] = regs[*reg] / op2
            },
            Instr::Mod(reg, op2) => {
                let op2 = decode_op2(op2, regs);
                regs[*reg] = regs[*reg] % op2
            },
            Instr::Eql(reg, op2) => {
                let op2 = decode_op2(op2, regs);
                regs[*reg] = if regs[*reg] == op2 {1} else {0}
            },
        }
    }
    regs
}

fn next (inp: &mut Vec<i64>) {

}

fn main() -> io::Result<()>{
    let monad = parse_input("./input.txt")?;
    pp_prog(&monad);
    println!("{:?}", run(&monad, &vec![9i64; 14]));

    Ok(())
}
