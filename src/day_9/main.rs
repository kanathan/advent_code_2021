use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    println!("Risk sum is {}",get_risk_sum(input));
    println!("Last 3 basin sizes multiplied are {}",get_basin_mult(input));
}

fn get_risk_sum(input: &str) -> usize {
    let heightmap = get_heightmap(input);

    let mut risk_sum = 0;
    for (x, y) in get_low_points(&heightmap) {
        risk_sum += heightmap[y][x] as usize + 1;
    }

    risk_sum
}

fn get_basin_mult(input: &str) -> usize {
    let heightmap = get_heightmap(input);

    let mut basin_sizes: Vec<usize> = get_low_points(&heightmap)
        .iter()
        .map(|p| get_basin_size(&heightmap, *p)).collect();
    basin_sizes.sort();

    basin_sizes.iter().rev().take(3).product()
}

fn get_basin_size(heightmap: &Vec<Vec<u8>>, low_point: (usize,usize)) -> usize {
    let mut basin_points: HashSet<(usize,usize)> = HashSet::new();
    
    fn get_nearby_basin_points(heightmap: &Vec<Vec<u8>>, existing_points: &mut HashSet<(usize, usize)>, point: (usize, usize)) {
        for new_point in get_adjacent_points(heightmap, point) {
            if heightmap[new_point.1][new_point.0] < 9 {
                if !existing_points.contains(&new_point) {
                    existing_points.insert(new_point);
                    get_nearby_basin_points(heightmap, existing_points, new_point);
                }
            }
        }
    }

    basin_points.insert(low_point);
    get_nearby_basin_points(heightmap, &mut basin_points, low_point);

    basin_points.len()
}

fn get_low_points(heightmap: &Vec<Vec<u8>>) -> Vec<(usize,usize)> {
    let mut low_points = Vec::new();

    for y in 0..heightmap.len() {
        for x in 0..heightmap[y].len() {
            let mut low_point = true;
            for (adj_x, adj_y) in get_adjacent_points(heightmap, (x,y)) {
                low_point &= heightmap[adj_y][adj_x] > heightmap[y][x];
            }
            if low_point {
                low_points.push((x,y));
            }
        }
    }

    low_points
}

fn get_adjacent_points(heightmap: &Vec<Vec<u8>>, point: (usize, usize)) -> Vec<(usize,usize)> {
    let mut adjacent_points = Vec::new();

    let x = point.0;
    let y = point.1;

    if x > 0 {
        adjacent_points.push((x-1, y));
    }
    if y > 0 {
        adjacent_points.push((x, y-1));
    }
    if x < heightmap[y].len()-1 {
        adjacent_points.push((x+1, y));
    }
    if y < heightmap.len()-1 {
        adjacent_points.push((x, y+1));
    }

    adjacent_points
}

fn get_heightmap(input: &str) -> Vec<Vec<u8>> {
    let mut heightmap: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        heightmap.push(line.chars().map(|c| c.to_digit(10).expect("Invalid input") as u8).collect());
    }
    heightmap
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
    "2199943210\n\
    3987894921\n\
    9856789892\n\
    8767896789\n\
    9899965678";

    #[test]
    fn test1() {
        assert_eq!(get_risk_sum(INPUT), 15)
    }

    #[test]
    fn test2() {
        let heightmap = get_heightmap(INPUT);
        let basin_sizes: Vec<usize> = get_low_points(&heightmap).iter().map(|p| get_basin_size(&heightmap, *p)).collect();
        let valid_sizes = [3, 9, 14, 9];
        assert_eq!(basin_sizes, valid_sizes)
    }

    #[test]
    fn test3() {
        assert_eq!(get_basin_mult(INPUT), 1134)
    }

}