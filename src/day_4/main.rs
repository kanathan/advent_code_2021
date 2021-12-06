use std::{collections::HashSet, iter::FromIterator};
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    let (last_draw, score) = get_winning_board(input);
    println!("Winning score: {}",last_draw * score);

    let (last_draw, score) = get_losing_board(input);
    println!("Losing board score: {}",last_draw * score);
}


#[derive(Debug)]
struct Board {
    entries: Vec<HashSet<i64>>,
    winning_score: Option<i64>,
}

impl Board {
    fn new(input: &str) -> Self {
        let mut entries = Vec::new();
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)").unwrap();
        }

        //The first 5 sets handle each column
        for _ in 0..5 {
            entries.push(HashSet::new());
        }

        for line in input.lines() {
            let captures = RE.captures(line).unwrap();
            let mut row = Vec::new();
            for idx in 1..6 {
                row.push(captures.get(idx).unwrap().as_str().parse::<i64>().unwrap())
            }
            for (idx, val) in row.iter().enumerate() {
                entries[idx].insert(*val);
            }
            entries.push(HashSet::from_iter(row.iter().cloned()));
        }

        Self {
            entries,
            winning_score: None,
        }
    }

    fn add_draw(&mut self, draw: i64) {
        for entry_set in self.entries.iter_mut() {
            entry_set.remove(&draw);
        }

        if self.entries.iter().any(|x| x.is_empty()) {
            let mut score = 0;
            for idx in 0..5 {
                let entry_set = self.entries.get(idx).unwrap();
                for val in entry_set {
                    score += val;
                }
            }
            self.winning_score = Some(score);
        }
    }

    fn get_score(&self) -> Option<i64> {
        self.winning_score
    }
}



fn get_winning_board(input: &str) -> (i64, i64) {
    let mut lines = input.lines();

    // Get draws
    let line = lines.next().unwrap();
    let mut draws = Vec::new();
    for draw_str in line.split(",") {
        draws.push(draw_str.parse::<i64>().expect("Unable to parse input"));
    }

    // Get boards
    let mut boards = Vec::new();
    lines.next();
    let mut board_input = String::new();
    for line in lines {
        if line.is_empty() {
            boards.push(Board::new(&board_input));
            board_input = String::new();
        }
        else {
            board_input.push_str(line);
            board_input.push_str("\n");
        }
    }
    boards.push(Board::new(&board_input));

    for draw in draws {
        for board in boards.iter_mut() {
            board.add_draw(draw);
            if let Some(score) = board.get_score() {
                return (draw, score);
            }
        }
    }

    (-1, 0)
}



fn get_losing_board(input: &str) -> (i64, i64) {
    let mut lines = input.lines();

    // Get draws
    let line = lines.next().unwrap();
    let mut draws = Vec::new();
    for draw_str in line.split(",") {
        draws.push(draw_str.parse::<i64>().expect("Unable to parse input"));
    }

    // Get boards
    let mut boards = Vec::new();
    lines.next();
    let mut board_input = String::new();
    for line in lines {
        if line.is_empty() {
            boards.push(Board::new(&board_input));
            board_input = String::new();
        }
        else {
            board_input.push_str(line);
            board_input.push_str("\n");
        }
    }
    boards.push(Board::new(&board_input));

    for draw in draws {
        if boards.len() == 1 {
            for board in boards.iter_mut() {
                board.add_draw(draw);
                if let Some(score) = board.get_score() {
                    return (draw, score);
                }
            }
        }
        else {
            for board in boards.iter_mut() {
                board.add_draw(draw);
            }
    
            boards.retain(|x| x.get_score().is_none());
        }
    }

    (-1, 0)
}



#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
        \n\
        22 13 17 11  0\n\
         8  2 23  4 24\n\
        21  9 14 16  7\n\
         6 10  3 18  5\n\
         1 12 20 15 19\n\
         \n\
         3 15  0  2 22\n\
         9 18 13 17  5\n\
        19  8  7 25 23\n\
        20 11 10 24  4\n\
        14 21 16 12  6\n\
        \n\
        14 21 17 24  4\n\
        10 16 15  9 19\n\
        18  8 23 26 20\n\
        22 11 13  6  5\n\
         2  0 12  3  7";


    #[test]
    fn test1() {
        assert_eq!(get_winning_board(INPUT), (24, 188));
    }

    #[test]
    fn test2() {
        assert_eq!(get_losing_board(INPUT), (13, 148));
    }
}