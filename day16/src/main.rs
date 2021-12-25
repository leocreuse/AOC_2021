//  AOC Day 16

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::collections::HashSet;
use std::collections::HashMap;

struct LitteralPacket {
    ver: u64,
    val: u64
}

struct OpPacket {
    ver: u64,
    typ: u64,
    len_typ: bool,
    len: u64,
    content: Vec<Packet>
}

enum Packet {
    Lit (LitteralPacket),
    Op (OpPacket)
}

fn parse_input (filename: &str) -> io::Result<Vec<char>> {
    let convert_map: HashMap<char, [char; 4]> = HashMap::from([
        ('0',['0','0','0','0']),
        ('1',['0','0','0','1']),
        ('2',['0','0','1','0']),
        ('3',['0','0','1','1']),
        ('4',['0','1','0','0']),
        ('5',['0','1','0','1']),
        ('6',['0','1','1','0']),
        ('7',['0','1','1','1']),
        ('8',['1','0','0','0']),
        ('9',['1','0','0','1']),
        ('A',['1','0','1','0']),
        ('B',['1','0','1','1']),
        ('C',['1','1','0','0']),
        ('D',['1','1','0','1']),
        ('E',['1','1','1','0']),
        ('F',['1','1','1','1'])]);
    let mut res: Vec<char> = Vec::new();
    let input = Path::new(filename);
    let file = File::open(input)?;
    let br = io::BufReader::new(file);
    let line = br.lines().next().expect("empty input")?;
    for chr in line.chars() {
        res.extend_from_slice(&convert_map[&chr]);
    }

    Ok(res)
}

fn to_u32 (buf: &[char]) -> u64 {
    let buf_str: String = buf.iter().collect();
    u64::from_str_radix(&buf_str, 2).expect("incorrect bin string")
}

fn translate_packet (buffer: &Vec<char>, idx: usize) -> (usize, Packet) {
    //  print!("parsing new packet: ");
    // for chr in buffer[idx ..].iter() {
    //     print!("{}", chr);
    // }
    // println!("");
    let mut cur_idx = idx;
    let version = to_u32(&buffer[cur_idx .. cur_idx + 3]);
    cur_idx +=3;
    let packet_typ = to_u32(&buffer[cur_idx .. cur_idx + 3]);
    cur_idx += 3;
    //  println!("type {}", packet_typ);
    if packet_typ == 4 {
        let mut res_pak = LitteralPacket{ver: version, val:0};
        let mut val_str: String = String::from("");
        let mut last = false;
        while !last {
            let group = &buffer[cur_idx .. cur_idx + 5];
            if group[0] == '0' {
                last = true;
            }
            val_str.extend(group[1..].iter());
            cur_idx += 5;
        }
        // println!("{}", val_str);
        res_pak.val = to_u32 (&val_str.chars().collect::<Vec<char>>());

        (cur_idx, Packet::Lit(res_pak))
    } else {
        let mut res = OpPacket {ver:version, typ: packet_typ, len_typ: false, len: 0, content: Vec::new()};
        let len_typ = buffer[cur_idx] == '1';
        cur_idx +=1;
        if len_typ {
            res.len = to_u32 (&buffer[cur_idx .. cur_idx + 11]);
            cur_idx += 11;
            for _ in 0 .. res.len {
                let (new_idx, new_packet) = translate_packet(buffer, cur_idx);
                res.content.push(new_packet);
                cur_idx = new_idx;
            }
        } else {
            res.len = to_u32(&buffer[cur_idx .. cur_idx + 15]);
            cur_idx += 15;
            let start_bit = cur_idx;
            while cur_idx < start_bit + (res.len as usize) {
                let (new_idx, new_packet) = translate_packet(buffer, cur_idx);
                res.content.push(new_packet);
                cur_idx = new_idx;
            }
        }
        (cur_idx, Packet::Op(res))
    }

}

fn sum_version_num (packet: &Packet) -> u64 {
    match packet {
        Packet::Lit (lit_pack) => lit_pack.ver,
        Packet::Op (op_pack) => op_pack.content.iter().fold(op_pack.ver, |acc, pckt| acc + sum_version_num(pckt))
    }
}

fn eval (packet: &Packet) -> u64 {
    match packet {
        Packet::Lit(pck) => pck.val,
        Packet::Op(pck) => {
            match pck.typ {
                0 => pck.content.iter().fold(0, |acc, pckt| acc + eval(pckt)),
                1 => pck.content.iter().fold(1, |acc, pckt| acc * eval(pckt)),
                2 => pck.content.iter().map(|pckt| eval(pckt)).min().expect("not element!!"),
                3 => pck.content.iter().map(|pckt| eval(pckt)).max().expect("not element!!"),
                4 => panic!("should not have a 4 here..."),
                5 => if eval (&pck.content[0]) > eval(&pck.content[1]) {1} else {0},
                6 => if eval (&pck.content[0]) < eval(&pck.content[1]) {1} else {0},
                7 => if eval (&pck.content[0]) == eval(&pck.content[1]) {1} else {0},
                _ => panic!("unknown op code")
            }
        }
    }
}

fn main() -> io::Result<()>{
    let bit_buff = parse_input("./input.txt")?;
    let (_, packet) = translate_packet(&bit_buff, 0);
    println!("Version number sum: {}", sum_version_num(&packet));
    println!("The packet we got evaluates to {}", eval(&packet));
    Ok(())
}
