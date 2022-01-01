use std::usize;

use ndarray::Array2;


fn main() {
    let input = include_str!("input");

    let (algorithm, grid) = parse_input(input, 2);
    println!("{}",get_lit_pixels(&algorithm, grid, 2));

    let (algorithm, grid) = parse_input(input, 50);
    println!("{}",get_lit_pixels(&algorithm, grid, 50));
}



fn parse_input(input: &str, steps: usize) -> (Vec<u8>, Array2<u8>) {
    let mut lines = input.lines();
    let alg: Vec<u8> = lines.next().unwrap().chars().map(|c| if c=='#' {1} else {0}).collect();

    lines.next();
    let lines_vec: Vec<&str> = lines.collect();
    let rows = lines_vec.len() + steps * 2 + 2;
    let cols = lines_vec[0].len() + steps * 2 + 2;

    let mut grid = Array2::<u8>::zeros((rows, cols));

    for (row, line) in lines_vec.into_iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            grid[[row+steps+1,col+steps+1]] = if c == '#' {1} else {0};
        }
    }


    (alg, grid)
}

fn get_lit_pixels(algorithm: &Vec<u8>, mut grid: Array2<u8>, steps: usize) -> usize {
    for _ in 0..steps {
        grid = step(algorithm, &grid);
    }

    let mut sum = 0;
    for val in grid.iter() {
        sum += *val as usize;
    }
    sum
}

fn step(algorithm: &Vec<u8>, grid: &Array2<u8>) -> Array2<u8> {
    let mut new_grid = grid.clone();
    for row in 0..(grid.shape()[0]) {
        for col in 0..(grid.shape()[1]) {
            let mut idx = 0;
            for inner_row in (row as isize-1)..=(row as isize)+1 {
                for inner_col in (col as isize)-1..=(col as isize)+1 {
                    if inner_col < 0 || inner_row < 0 {
                        idx = (idx << 1) | (grid[[0,0]] as usize);
                    } else if let Some(val) = grid.get([inner_row as usize, inner_col as usize]) {
                        idx = (idx << 1) | (*val as usize);
                    } else {
                        idx = (idx << 1) | (grid[[0,0]] as usize);
                    }
                }
            }
            new_grid[[row,col]] = algorithm[idx];
        }
    };
    new_grid
}



#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
    ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
    #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
    .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
    .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
    .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
    ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
    ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n\
    \n\
    #..#.\n\
    #....\n\
    ##..#\n\
    ..#..\n\
    ..###";

    #[test]
    fn test1() {
        let (algorithm, grid) = parse_input(INPUT, 2);

        println!("{:?}",algorithm);
        println!();
        println!("{:?}",grid);

        println!("{}",get_lit_pixels(&algorithm, grid, 2));

        let (algorithm, grid) = parse_input(INPUT, 50);
        println!("{}",get_lit_pixels(&algorithm, grid, 50));
    }
}