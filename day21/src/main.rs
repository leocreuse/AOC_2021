
struct Player {
    position: u32,
    current_score: u32
}

impl Player {
    fn play(&mut self, dice: &mut Dice) -> bool {
        self.position = (self.position - 1 + dice.throw() + dice.throw() + dice.throw())%10 + 1;
        self.current_score += self.position;
        self.current_score >= 1000
    }
}

struct Dice {
    throw_count: u32,
    last_throw: u32,
    max: u32
}

impl Dice {
    fn throw (&mut self) -> u32 {
        self.throw_count += 1;
        self.last_throw = (self.last_throw) % self.max + 1;
        self.last_throw
    }
}

fn part1 () {
    let mut players: Vec<Player> = vec![Player{position:7, current_score:0}, Player{position:1, current_score:0}];
    let mut dice = Dice{throw_count: 0, last_throw:0, max: 100};
    let mut current_player: usize = 0;
    while ! players[current_player % 2].play(&mut dice) {
        current_player +=1;
    }
    println!("Player {} won!: {}, {}, {}", current_player%2 + 1, dice.throw_count, dice.throw_count * players[(current_player + 1) % 2].current_score, players[(current_player + 1) % 2].current_score);

}

fn play (player: usize, scores: [u64; 2], pos: [u64; 2]) -> (u64, u64) {
    let mut win1: u64 = 0;
    let mut win2: u64 = 0;
    let play_idx = player % 2;
    let nb_univ: [u64; 7] = [1, 3, 6, 7, 6, 3, 1];
    for i in 0usize .. 7 {
        let mut new_scores = scores.clone();
        let mut new_pos = pos.clone();
        new_pos[play_idx] = (new_pos[play_idx] + i as u64 + 2) % 10 + 1;
        new_scores[play_idx] += new_pos[play_idx];
        if new_scores[play_idx] >= 21 {
            if play_idx == 0 {
                win1 += nb_univ[i];
            } else {
                win2 += nb_univ[i];
            }
        } else {
            let (univ1, univ2) = play(player + 1, new_scores, new_pos);
            win1 += nb_univ[i] * univ1;
            win2 += nb_univ[i] * univ2;
        }
    }
    (win1, win2)
}

fn part2() {
    let (win1, win2) = play(0, [0,0], [7,1]);
    println!("player 1 univers win: {}, player 2 univers win: {}, Player one bigger: {}", win1, win2, win1 >= win2);
}

fn main() {
    part1();
    part2();
}
