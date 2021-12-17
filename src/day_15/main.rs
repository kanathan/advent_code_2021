use ndarray::Array2;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = include_str!("input");
    
    let grid = parse_input(input);
    println!("{}",get_lowest_risk(&grid));

    let grid = parse_input_large(input);
    println!("{}",get_lowest_risk(&grid));

}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    coords: Coords,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_lowest_risk(grid: &Array2<u32>) -> u32 {
    let start_coords = Coords { x: 0, y: 0 };
    let end_coords = Coords { x: grid.shape()[0]-1, y: grid.shape()[1]-1 };

    let mut frontier = BinaryHeap::new();
    frontier.push(State {cost: 0, coords: start_coords });
    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();
    came_from.insert(start_coords, start_coords);
    cost_so_far.insert(start_coords, 0);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();

        if current.coords == end_coords { break }

        for neighbor in get_neighbors(current.coords) {
            if let Some(cost) = grid.get((neighbor.x, neighbor.y)) {
                let new_cost = cost_so_far[&current.coords] + cost;
                if !cost_so_far.contains_key(&neighbor) || new_cost < cost_so_far[&neighbor] {
                    cost_so_far.insert(neighbor, new_cost);
                    frontier.push(State {cost: new_cost, coords: neighbor});
                    came_from.insert(neighbor, current.coords);
                }

            }
        }
    }

    cost_so_far[&end_coords]
}

fn get_neighbors(coords: Coords) -> Vec<Coords> {
    let mut neighbors = Vec::new();
    if coords.x > 0 {
        neighbors.push(Coords { x: coords.x-1, y: coords.y });
    }
    if coords.y > 0 {
        neighbors.push(Coords { x: coords.x, y: coords.y-1 });
    }
    neighbors.push(Coords { x: coords.x+1, y: coords.y });
    neighbors.push(Coords { x: coords.x, y: coords.y+1 });

    neighbors
}

fn parse_input(input: &str) -> Array2<u32> {
    let dims = (input.lines().next().unwrap().len(), input.lines().count());
    let mut grid = Array2::zeros(dims);

    for (y, row) in input.lines().enumerate() {
        for (x, val) in row.chars().enumerate() {
            grid[[x,y]] = val.to_digit(10).unwrap();
        }
    }

    grid
}

fn parse_input_large(input: &str) -> Array2<u32> {
    let dims = (input.lines().next().unwrap().len(), input.lines().count());
    let grid_dims = (dims.0*5, dims.1*5);
    let mut grid = Array2::zeros(grid_dims);

    for (y, row) in input.lines().enumerate() {
        for (x, val_c) in row.chars().enumerate() {
            let val = val_c.to_digit(10).unwrap();
            for seg_y in 0..5 {
                let y_offset = dims.1 * (seg_y as usize);
                for seg_x in 0..5 {
                    let mut seg_val = val + seg_x + seg_y;
                    while seg_val >= 10 {
                        seg_val -= 9;
                    }
                    let x_offset = dims.0 * (seg_x as usize);
                    grid[[x+x_offset,y+y_offset]] = seg_val;
                }
            }
        }
    }

    grid
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
    "1163751742\n\
    1381373672\n\
    2136511328\n\
    3694931569\n\
    7463417111\n\
    1319128137\n\
    1359912421\n\
    3125421639\n\
    1293138521\n\
    2311944581";

    #[test]
    fn test1() {
        let grid = parse_input(INPUT);
        assert_eq!(get_lowest_risk(&grid), 40);
    }

    #[test]
    fn test2() {
        let grid = parse_input_large(INPUT);
        assert_eq!(get_lowest_risk(&grid), 315);
    }
}