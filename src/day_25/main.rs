use std::fmt::Debug;

use ndarray::Array2;

fn main() {
    let input = include_str!("input");
    let mut board = parse(input);
    let mut count = 1;
    while step(&mut board) {
        count += 1;
    }
    println!("{}",count)
}

fn step(board: &mut Array2<Space>) -> bool {
    let mut did_move = false;
    let row_size = board.shape()[0];
    let col_size = board.shape()[1];
    
    // Move east
    let mut to_move = Vec::new();
    for row in 0..row_size {
        for col in 0..col_size {
            if board[[row,col]] == Space::East && board[[row, (col+1)%col_size]] == Space::None {
                to_move.push(([row,col], [row, (col+1)%col_size]));
            }
        }
    }
    did_move |= !to_move.is_empty();
    for (from_coord, to_coord) in to_move.into_iter() {
        board[from_coord] = Space::None;
        board[to_coord] = Space::East;
    }

    // Move south
    let mut to_move = Vec::new();
    for row in 0..row_size {
        for col in 0..col_size {
            if board[[row,col]] == Space::South && board[[(row+1)%row_size,col]] == Space::None {
                to_move.push(([row,col], [(row+1)%row_size,col]));
            }
        }
    }
    did_move |= !to_move.is_empty();
    for (from_coord, to_coord) in to_move.into_iter() {
        board[from_coord] = Space::None;
        board[to_coord] = Space::South;
    }

    did_move
}

fn parse(input: &str) -> Array2<Space> {
    let shape = [input.lines().count(), input.lines().next().unwrap().chars().count()];
    
    let vec = input.chars()
        .filter_map(|c| match c {
            '.' => Some(Space::None),
            '>' => Some(Space::East),
            'v' => Some(Space::South),
            _ => None,
        })
        .collect::<Vec<Space>>();

    Array2::from_shape_vec(shape, vec).unwrap()
}

#[derive(PartialEq, Eq)]
enum Space {
    None,
    East,
    South,
}

impl Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "."),
            Self::East => write!(f, ">"),
            Self::South => write!(f, "v"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "v...>>.vv>\n\
    .vv>>.vv..\n\
    >>.>v>...v\n\
    >>v>>.>.v.\n\
    v>v.vv.v..\n\
    >.>>..v...\n\
    .vv..>.>v.\n\
    v.v..>>v.v\n\
    ....v..v.>";

    #[test]
    fn test1() {
        let mut board = parse(INPUT);
        let mut count = 1;
        while step(&mut board) {
            count += 1;
            if count > 100 { panic!() }
        }
        println!("{}",count)
    }
}