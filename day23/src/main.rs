
#[derive(Debug,PartialEq, Eq, Clone, Copy)]
enum AmphiType {
    A, B, C, D
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Amphipode {
    typ: AmphiType,
    energy: i32,
    x: usize,
    y: usize,
    moved_to_corr: bool,
    moved_to_room: bool
}

impl Amphipode {
    fn room_x (&self) -> usize {
        match self.typ {
            AmphiType::A => 2,
            AmphiType::B => 4,
            AmphiType::C => 6,
            AmphiType::D => 8
        }
    }

    fn cost_per_move (&self) -> i32 {
        match self.typ {
            AmphiType::A => 1,
            AmphiType::B => 10,
            AmphiType::C => 100,
            AmphiType::D => 1000
        }
    }

    fn to_char(&self) -> char {
        match self.typ {
            AmphiType::A => 'A',
            AmphiType::B => 'B',
            AmphiType::C => 'C',
            AmphiType::D => 'D'
        }
    }
    
    fn possible_moves(&self, others: &Vec<Amphipode>) -> Vec<(usize, usize)> {
        let mut res: Vec<(usize, usize)> = Vec::new();
        if !self.moved_to_corr {
            let mut min_x: usize = 0;
            let mut max_x: usize = 10;
            let mut above_free: bool = true;
            for amphi in others {
                if self != amphi {
                    if self.x == amphi.x && self.y > amphi.y{
                        above_free = false;
                        break;
                    }
                    if amphi.y ==0 && amphi.x < self.x && amphi.x >= min_x {
                        min_x = amphi.x + 1;
                    }
                    if amphi.y ==0 && amphi.x > self.x && amphi.x <= max_x {
                        max_x = amphi.x - 1;
                    }
                }
            }
            if above_free {
                for x in min_x .. max_x + 1 {
                    if !above_room(x) {
                        res.push((x, 0));
                    }
                }
            }
        } else if !self.moved_to_room {
            let target = self.room_x();
            let xmin = usize::min(target, self.x);
            let xmax = usize::max(target, self.x);
            let mut room_free: [bool; 5] = [true; 5];
            for amphi in others {
                if amphi != self {
                    if amphi.x == target {
                        room_free[amphi.y] = false;
                        if amphi.typ != self.typ {
                            room_free[0] = false;
                        }
                    }
                    if amphi.y == 0 && amphi.x <= xmax && amphi.x >= xmin{
                        room_free[0] = false;
                    }
                    if !room_free[0] {
                        break;
                    }
                }
            }
            if room_free[0] && room_free[1]{
                let mut free_idx = 1usize;
                while free_idx < 5 && room_free[free_idx] {
                    free_idx +=1;
                }
                res.push((target, free_idx - 1));
            }
        }
        res
    }

    fn do_move(&self, mov: (usize, usize)) -> (Amphipode, i32) {
        let move_cost =  self.cost_per_move() * (i32::abs(mov.1 as i32 - self.y as i32) + i32::abs(mov.0 as i32 - self.x as i32));
        let energy = self.energy + move_cost;
        let moved_to_corr: bool = true;
        let moved_to_room: bool = self.moved_to_corr;
        (Amphipode { typ: self.typ, energy, x: mov.0, y: mov.1, moved_to_corr, moved_to_room}, move_cost)
    }
}

fn check_final_pos (amphis: &Vec<Amphipode>) -> bool {
    let mut res = true;
    for a in amphis {
        if a.x != a.room_x() {
            res = false;
            break;
        }
    }
    res
}

#[inline(always)]
fn above_room (x: usize) -> bool {
    x == 2 || x == 4 || x == 6 || x == 8
}

fn pp_board (amphis: &Vec<Amphipode>) {
    let corridor: Vec<char> = "#...........#".chars().collect();
    let room0: Vec<char> = "###.#.#.#.###".chars().collect();
    let room1: Vec<char> = "  #.#.#.#.#  ".chars().collect();
    let room2: Vec<char> = "  #.#.#.#.#  ".chars().collect();
    let room3: Vec<char> = "  #.#.#.#.#  ".chars().collect();
    let mut board: Vec<Vec<char>> = vec![corridor,room0,room1, room2, room3];
    for amphi in amphis {
        board[amphi.y][amphi.x + 1] = amphi.to_char();
    }
    println!("#############");
    for line in board {
        for chr in line {
            print!("{}", chr);
        }
        println!("");
    }
    println!("  #########  ");
}

fn play (amphis: &Vec<Amphipode>, partial_score: i32, min_score: i32) -> i32 {
    let mut score = min_score;
    let mut game_over: bool = true;
    for (idx,amphi) in amphis.iter().enumerate() {
        let moves = amphi.possible_moves(amphis);
        for mov in moves {
            game_over = false;
            let (new_amphi, move_cost) = amphi.do_move(mov);
            if partial_score + move_cost <= score {
                let mut new_amphis = amphis.clone();
                new_amphis[idx] = new_amphi;
                // println!("Amphi {}, from {},{} to {},{} ", amphi.to_char(), amphi.x, amphi.y, mov.0, mov.1);
                // pp_board(&new_amphis);
                let move_score = play(&new_amphis, partial_score + move_cost, score);
                if move_score < score {
                    score = move_score;
                }
            }
        }
    }
    if game_over {
        if check_final_pos(amphis) {
            score = amphis.iter().fold(0, |acc, a| acc + a.energy);
            // println!("{}", score);
        }
    }
    score
}

fn main() {
    let mut amphis: Vec<Amphipode> = Vec::new();

    // amphis.push(Amphipode {typ: AmphiType::B, energy: 0, x: 2, y: 1, moved_to_corr: false, moved_to_room: false});
    // amphis.push(Amphipode {typ: AmphiType::A, energy: 0, x: 2, y: 4, moved_to_corr: true, moved_to_room: true});
    // amphis.push(Amphipode {typ: AmphiType::C, energy: 0, x: 4, y: 1, moved_to_corr: false, moved_to_room: false});
    // amphis.push(Amphipode {typ: AmphiType::D, energy: 0, x: 4, y: 4, moved_to_corr: false, moved_to_room: false});
    // amphis.push(Amphipode {typ: AmphiType::B, energy: 0, x: 6, y: 1, moved_to_corr: false, moved_to_room: false});
    // amphis.push(Amphipode {typ: AmphiType::C, energy: 0, x: 6, y: 4, moved_to_corr: true, moved_to_room: true});
    // amphis.push(Amphipode {typ: AmphiType::D, energy: 0, x: 8, y: 1, moved_to_corr: false, moved_to_room: false});
    // amphis.push(Amphipode {typ: AmphiType::A, energy: 0, x: 8, y: 4, moved_to_corr: false, moved_to_room: false});

    // amphis.push(Amphipode {typ: AmphiType::B, energy: 0, x: 2, y: 1, moved_to_corr: false, moved_to_room: false});
    // amphis.push(Amphipode {typ: AmphiType::B, energy: 0, x: 2, y: 4, moved_to_corr: false, moved_to_room: false});
    // amphis.push(Amphipode {typ: AmphiType::C, energy: 0, x: 4, y: 1, moved_to_corr: false, moved_to_room: false});
    // amphis.push(Amphipode {typ: AmphiType::C, energy: 0, x: 4, y: 4, moved_to_corr: false, moved_to_room: false});
    // amphis.push(Amphipode {typ: AmphiType::A, energy: 0, x: 6, y: 1, moved_to_corr: false, moved_to_room: false});
    // amphis.push(Amphipode {typ: AmphiType::D, energy: 0, x: 6, y: 4, moved_to_corr: false, moved_to_room: false});
    // amphis.push(Amphipode {typ: AmphiType::D, energy: 0, x: 8, y: 1, moved_to_corr: false, moved_to_room: false});
    // amphis.push(Amphipode {typ: AmphiType::A, energy: 0, x: 8, y: 4, moved_to_corr: false, moved_to_room: false});

    amphis.push(Amphipode {typ: AmphiType::D, energy: 0, x: 2, y: 1, moved_to_corr: false, moved_to_room: false});
    amphis.push(Amphipode {typ: AmphiType::C, energy: 0, x: 2, y: 4, moved_to_corr: false, moved_to_room: false});
    amphis.push(Amphipode {typ: AmphiType::B, energy: 0, x: 4, y: 1, moved_to_corr: false, moved_to_room: false});
    amphis.push(Amphipode {typ: AmphiType::A, energy: 0, x: 4, y: 4, moved_to_corr: false, moved_to_room: false});
    amphis.push(Amphipode {typ: AmphiType::A, energy: 0, x: 6, y: 1, moved_to_corr: false, moved_to_room: false});
    amphis.push(Amphipode {typ: AmphiType::D, energy: 0, x: 6, y: 4, moved_to_corr: false, moved_to_room: false});
    amphis.push(Amphipode {typ: AmphiType::C, energy: 0, x: 8, y: 1, moved_to_corr: false, moved_to_room: false});
    amphis.push(Amphipode {typ: AmphiType::B, energy: 0, x: 8, y: 4, moved_to_corr: false, moved_to_room: false});

    amphis.push(Amphipode {typ: AmphiType::D, energy: 0, x: 2, y: 2, moved_to_corr: false, moved_to_room: false});
    amphis.push(Amphipode {typ: AmphiType::D, energy: 0, x: 2, y: 3, moved_to_corr: false, moved_to_room: false});
    amphis.push(Amphipode {typ: AmphiType::C, energy: 0, x: 4, y: 2, moved_to_corr: false, moved_to_room: false});
    amphis.push(Amphipode {typ: AmphiType::B, energy: 0, x: 4, y: 3, moved_to_corr: false, moved_to_room: false});
    amphis.push(Amphipode {typ: AmphiType::B, energy: 0, x: 6, y: 2, moved_to_corr: false, moved_to_room: false});
    amphis.push(Amphipode {typ: AmphiType::A, energy: 0, x: 6, y: 3, moved_to_corr: false, moved_to_room: false});
    amphis.push(Amphipode {typ: AmphiType::A, energy: 0, x: 8, y: 2, moved_to_corr: false, moved_to_room: false});
    amphis.push(Amphipode {typ: AmphiType::C, energy: 0, x: 8, y: 3, moved_to_corr: false, moved_to_room: false});

    pp_board(&amphis);
    let min_score = play(&amphis, 0, i32::max_value());
    println!("Minimal score: {}", min_score);
    
}
