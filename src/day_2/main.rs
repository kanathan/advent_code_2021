use lazy_static::lazy_static;
use regex::Regex;

enum Direction {
    Up,
    Down,
    Forward,
}



fn main() {
    let input = include_str!("input.txt");
    
    let pos = get_pos(input);
    println!("Final pos: {} x {} = {}",pos.0, pos.1, pos.0 * pos.1);

    let pos = get_pos_advanced(input);
    println!("Final adv pos: {} x {} = {}",pos.0, pos.1, pos.0 * pos.1);
}

fn parse_string(line: &str) -> (Direction, i64) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\w+) (\d+)").unwrap();
    }
    
    let captures = RE.captures(line).unwrap();
    let dist = captures.get(2).unwrap().as_str().parse().unwrap();

    match captures.get(1).unwrap().as_str() {
        "up" => (Direction::Up, dist),
        "down" => (Direction::Down, dist),
        "forward" => (Direction::Forward, dist),
        _ => panic!("Invalid input: {}",line)
    }
}

fn get_pos(input: &str) -> (i64, i64) {
    let mut h_pos = 0;
    let mut depth = 0;

    for line in input.lines() {
        let (dir, dist) = parse_string(line);
        match dir {
            Direction::Up => depth -= dist,
            Direction::Down => depth += dist,
            Direction::Forward => h_pos += dist,
        }
    }

    (h_pos, depth)
}

fn get_pos_advanced(input: &str) -> (i64, i64) {
    let mut h_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input.lines() {
        let (dir, dist) = parse_string(line);
        match dir {
            Direction::Up => aim -= dist,
            Direction::Down => aim += dist,
            Direction::Forward => {
                h_pos += dist;
                depth = depth + aim * dist;
            }
        }
    }

    (h_pos, depth)
}





#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
        "forward 5\n\
        down 5\n\
        forward 8\n\
        up 3\n\
        down 8\n\
        forward 2";

    #[test]
    fn test1() {
        assert_eq!(get_pos(INPUT), (15, 10));
    }

    #[test]
    fn test2() {
        assert_eq!(get_pos_advanced(INPUT), (15, 60));
    }
}