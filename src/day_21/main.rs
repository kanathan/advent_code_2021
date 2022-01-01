use std::collections::HashMap;

fn main() {
    println!("{}",play(10, 7));

    println!("{:?}",get_outcomes(10, 7));
}


fn play(p1_start: usize, p2_start: usize) -> usize {
    let mut p1_pos = p1_start - 1;
    let mut p2_pos = p2_start - 1;
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut die = Die::new();

    loop {
        // P1 turn
        let movement = die.roll() + die.roll() + die.roll();
        p1_pos = (p1_pos + movement) % 10;
        p1_score += p1_pos + 1;
        if p1_score >= 1000 { break }

        // P2 turn
        let movement = die.roll() + die.roll() + die.roll();
        p2_pos = (p2_pos + movement) % 10;
        p2_score += p2_pos + 1;
        if p2_score >= 1000 { break }
    }

    println!("Player 1: {}",p1_score);
    println!("Player 2: {}",p2_score);
    println!("Die rolls: {}",die.count);

    die.count * p1_score.min(p2_score)
}

fn get_outcomes(p1_start: usize, p2_start: usize) -> (usize, usize) {
    let p1 = Player { pos: p1_start-1, score: 0 };
    let p2 = Player { pos: p2_start-1, score: 0 };
    let mut win_map = HashMap::new();
    step(p1, p2, &mut win_map)
}

fn step(cur_player: Player, other_player: Player, win_map: &mut HashMap<(Player, Player), (usize, usize)>) -> (usize, usize) {
    if let Some(outcome) = win_map.get(&(cur_player, other_player)) {
        return *outcome
    }

    let mut wins = (0, 0);
    for roll1 in 1..=3 {
        for roll2 in 1..=3 {
            for roll3 in 1..=3 {
                let mut cur_player_copy = cur_player;
                let movement = roll1 + roll2 + roll3;
                cur_player_copy.pos = (cur_player_copy.pos + movement) % 10;
                cur_player_copy.score += cur_player_copy.pos + 1;

                if cur_player_copy.score >= 21 {
                    wins.0 += 1;
                } else {
                    let new_wins = step(other_player, cur_player_copy, win_map);
                    wins.0 += new_wins.1;
                    wins.1 += new_wins.0;
                }
            }
        }
    }

    win_map.insert((cur_player, other_player), wins);

    wins
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Player {
    pos: usize,
    score: usize,
}

struct Die {
    last_roll: usize,
    count: usize,
}

impl Die {
    fn new() -> Self {
        Self { last_roll: 0, count: 0 }
    }

    fn roll(&mut self) -> usize {
        self.count += 1;
        self.last_roll += 1;
        if self.last_roll > 100 { self.last_roll = 1 };
        self.last_roll
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        println!("{}",play(4, 8));

        println!("{:?}",get_outcomes(4, 8));
    }
}