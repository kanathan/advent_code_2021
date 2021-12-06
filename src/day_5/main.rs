use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    println!("Overlap of >=2: {}",get_overlap_count(2, input));
    println!("Diag overlap of >=2: {}",get_diag_overlap_count(2, input));
}


fn get_overlap_count(threshold: usize, input: &str) -> usize {
    let mut map = Map::new();

    for line in input.lines() {
        let ((x1, y1), (x2, y2)) = parse_line(line);
        map.add_line(x1, x2, y1, y2, false);
    }

    map.get_count(threshold)
}

fn get_diag_overlap_count(threshold: usize, input: &str) -> usize {
    let mut map = Map::new();

    for line in input.lines() {
        let ((x1, y1), (x2, y2)) = parse_line(line);
        map.add_line(x1, x2, y1, y2, true);
    }

    //map.print_map();
    map.get_count(threshold)
}



fn parse_line(line: &str) -> ((usize, usize), (usize, usize)) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    }

    let captures = RE.captures(line).unwrap();
    let x1 = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let y1 = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let x2 = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
    let y2 = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
    ((x1,y1),(x2,y2))
}


struct Map {
    data: Vec<Vec<usize>>,
    x_max: usize,
    y_max: usize,
}

impl Map {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            x_max: 0,
            y_max: 0,
        }
    }

    fn resize(&mut self, x: usize, y: usize) {
        if y+1 > self.y_max {
            while self.data.len() < y+1 {
                self.data.push(Vec::new());
            }
            self.y_max = y+1;
        }
        
        for row in self.data.iter_mut() {
            while row.len() < x+1 {
                row.push(0);
            }
        }
        if x+1 > self.x_max {
            self.x_max = x+1;
        }
    }

    fn add_line(&mut self, x1: usize, x2: usize, y1: usize, y2: usize, use_diag: bool) {
        if x1 == x2 {
            self.add_vert_line(x1, y1, y2);
        }
        else if y1 == y2 {
            self.add_horz_line(x1, x2, y1);
        }
        else if use_diag {
            self.add_diag_line(x1, x2, y1, y2)
        }
    }

    fn add_horz_line(&mut self, x1: usize, x2: usize, y: usize) {
        let x_min = x1.min(x2);
        let x_max = x1.max(x2);
        self.resize(x_max, y);

        for cur_x in x_min..x_max+1 {
            self.data[y][cur_x] += 1;
        }
    }

    fn add_vert_line(&mut self, x: usize, y1: usize, y2: usize) {
        let y_min = y1.min(y2);
        let y_max = y1.max(y2);
        self.resize(x, y_max);

        for cur_y in y_min..y_max+1 {
            self.data[cur_y][x] += 1;
        }
    }

    fn add_diag_line(&mut self, x1: usize, x2: usize, y1: usize, y2: usize) {
        struct Point {
            x: usize,
            y: usize,
        }

        let p1: Point;
        let p2: Point;
        if x1 <= x2 {
            p1 = Point { x: x1, y: y1 };
            p2 = Point { x: x2, y: y2 };
        } else {
            p1 = Point { x: x2, y: y2 };
            p2 = Point { x: x1, y: y1 };
        }
        let x_max = x1.max(x2);
        let y_max = y1.max(y2);
        self.resize(x_max, y_max);


        let mut cur_y = p1.y;
        for cur_x in p1.x..p2.x+1 {
            self.data[cur_y][cur_x] += 1;
            if p2.y > p1.y { cur_y += 1 } else if cur_y > 0 { cur_y -= 1 }
        }
    }

    fn get_count(&self, threshold: usize) -> usize {
        let mut count = 0;
        for row in self.data.iter() {
            for val in row {
                if *val >= threshold { count += 1 }
            }
        }
        count
    }

    #[allow(dead_code)]
    fn print_map(&self) {
        for row in self.data.iter() {
            for val in row {
                if *val != 0 { print!("{}",val) }
                else { print!(".") }
            }
            println!()
        }
    }
}





#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
        "0,9 -> 5,9\n\
        8,0 -> 0,8\n\
        9,4 -> 3,4\n\
        2,2 -> 2,1\n\
        7,0 -> 7,4\n\
        6,4 -> 2,0\n\
        0,9 -> 2,9\n\
        3,4 -> 1,4\n\
        0,0 -> 8,8\n\
        5,5 -> 8,2";


    #[test]
    fn test1() {
        assert_eq!(get_overlap_count(2, INPUT), 5);

    }

    #[test]
    fn test2() {
        assert_eq!(get_diag_overlap_count(2, INPUT), 12);
    }
}