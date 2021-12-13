use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    println!("Total dots at one fold: {}", get_dot_count_one_fold(input));
    println!("Final appearance after folding:");
    get_final_fold(input);
}


fn get_dot_count_one_fold(input: &str) -> usize {
    let (mut grid, inst) = parse_input(input);

    let (axis, pos) = inst.get(0).unwrap();
    fold(&mut grid, *axis, *pos);

    let mut count = 0;
    for row in grid.iter() {
        for val in row.iter() {
            if *val { count += 1 }
        }
    }
    count
}

fn get_final_fold(input: &str) {
    let (mut grid, inst) = parse_input(input);

    for (axis, pos) in inst.iter() {
        fold(&mut grid, *axis, *pos);
    }
    
    print_grid(&grid);    
}


fn fold(grid: &mut Vec<Vec<bool>>, axis: char, pos: usize) {
    match axis {
        'x' => {
            for row in grid.iter_mut() {
                let mut offset = 1;
                while pos >= offset && pos+offset < row.len() {
                    row[pos-offset] |= row[pos+offset];
                    offset += 1;
                }
                row.resize(pos, false);
            }
        },
        'y' => {
            let mut offset = 1;
            while pos >= offset && pos+offset < grid.len() {
                for x in 0..grid[pos-offset].len() {
                    grid[pos-offset][x] |= grid[pos+offset][x];
                }
                offset += 1;
            }
            grid.resize(pos, Vec::new());
        },
        _ => panic!("Unexpected axis: {}",axis)
    }
}


fn parse_input(input: &str) -> (Vec<Vec<bool>>, Vec<(char, usize)>) {
    lazy_static! {
        static ref RE_INST: Regex = Regex::new(r"^fold along ([xy])=(\d+)").unwrap();
    }

    let mut grid = Vec::new();
    let mut instructions = Vec::new();

    let mut lines = input.lines();
    loop {
        if let Some(line) = lines.next() {
            if line.is_empty() { break }
            let (x_str, y_str) = line.split_once(',').expect(&format!("Unable to parse {}",line));
            let x = x_str.parse::<usize>().unwrap();
            let y = y_str.parse::<usize>().unwrap();
            while grid.len() < y+1 { grid.push(Vec::new()) }
            let x_size = (x+1).max(grid[0].len());
            for row in grid.iter_mut() {
                while row.len() < x_size { row.push(false) }
            }
            grid[y][x] = true;
        }
    }
    
    for line in lines {
        let captures = RE_INST.captures(line).expect(&format!("Unable to parse: {}",line));
        let axis = captures.get(1).unwrap().as_str().chars().next().unwrap();
        let pos = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        instructions.push((axis, pos));
    }

    (grid, instructions)
}

fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid.iter() {
        for val in row {
            if *val { print!("#") } else { print!(".") }
        }
        println!()
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
    "6,10\n\
    0,14\n\
    9,10\n\
    0,3\n\
    10,4\n\
    4,11\n\
    6,0\n\
    6,12\n\
    4,1\n\
    0,13\n\
    10,12\n\
    3,4\n\
    3,0\n\
    8,4\n\
    1,10\n\
    2,14\n\
    8,10\n\
    9,0\n\
    \n\
    fold along y=7\n\
    fold along x=5";

    #[test]
    fn test1() {
        assert_eq!(get_dot_count_one_fold(INPUT),17);
    }


}