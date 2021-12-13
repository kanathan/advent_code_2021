

fn main() {
    let input = include_str!("input.txt");

    println!("Flashes after 100 steps: {}",get_total_flashes(input, 100));
    println!("Synced flashes at step {}",get_sync_step(input, 999999));
}

fn get_total_flashes(input: &str, steps: usize) -> usize {
    let mut grid = get_grid(input);
    let mut flash_count = 0;

    for _ in 0..steps {
        step_grid(&mut grid, &mut flash_count);
    }

    flash_count
}

fn get_sync_step(input: &str, max_steps: usize) -> usize {
    let mut grid = get_grid(input);
    let mut flash_count = 0;
    let mut step = 0;

    loop {
        step += 1;
        step_grid(&mut grid, &mut flash_count);
        if grid.iter().all(|row| row.iter().all(|val| *val == Some(0))) {
            return step
        }
        if step > max_steps { return 0 }
    }
}

/*fn get_steps(input: &str) {
    let mut grid = get_grid(input);
    let mut flash_count = 0;

    print_grid(&grid);

    for step in 0..100 {
        step_grid(&mut grid, &mut flash_count);
        if (step+1) % 10 == 0 {
            println!("\nStep {}",step+1);
            print_grid(&grid);
        }
    }

    println!("Total steps: {}",flash_count);

}

fn print_grid(grid: &Vec<Vec<Option<u8>>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if let Some(val) = grid[y][x] {
                if val > 0 { print!("{}",val) }
                else { print!("\u{001b}[33;1m{0}\u{001b}[0m", val) }
            }
        }
        println!();
    }
}*/


fn step_grid(grid: &mut Vec<Vec<Option<u8>>>, flash_count: &mut usize) {
    // Inc each by 1
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if let Some(ref mut val) = grid[y][x] { *val += 1; }
        }
    }

    let mut flash_incomplete = true;
    while flash_incomplete {
        flash_incomplete = false;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if let Some(val) = grid[y][x] {
                    if val > 9 {
                        // Flash
                        *flash_count += 1;
                        grid[y][x] = None;
                        for (adj_x, adj_y) in get_adjacent_points(grid, (x,y)) {
                            if let Some(ref mut adj_val) = grid[adj_y][adj_x] {
                                *adj_val += 1;
                            }
                        }
                        flash_incomplete = true;
                    }
                }
            }
        }
    }
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x].is_none() { grid[y][x] = Some(0) }
        }
    }
}


fn get_grid(input: &str) -> Vec<Vec<Option<u8>>> {
    let mut grid: Vec<Vec<Option<u8>>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().map(|c| Some(c.to_digit(10).expect("Invalid input") as u8)).collect());
    }
    grid
}

fn get_adjacent_points(grid: &Vec<Vec<Option<u8>>>, point: (usize, usize)) -> Vec<(usize,usize)> {
    let mut adjacent_points = Vec::new();

    let x = point.0;
    let y = point.1;

    if x > 0 {
        adjacent_points.push((x-1, y));
    }
    if y > 0 {
        adjacent_points.push((x, y-1));
    }
    if x < grid[y].len()-1 {
        adjacent_points.push((x+1, y));
    }
    if y < grid.len()-1 {
        adjacent_points.push((x, y+1));
    }

    if x > 0 && y > 0 {
        adjacent_points.push((x-1, y-1));
    }
    if x > 0 && y < grid.len()-1 {
        adjacent_points.push((x-1, y+1));
    }
    if x < grid[y].len()-1 && y > 0 {
        adjacent_points.push((x+1, y-1));
    }
    if x < grid[y].len()-1 && y < grid.len()-1 {
        adjacent_points.push((x+1, y+1));
    }

    adjacent_points
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
    "5483143223\n\
    2745854711\n\
    5264556173\n\
    6141336146\n\
    6357385478\n\
    4167524645\n\
    2176841721\n\
    6882881134\n\
    4846848554\n\
    5283751526";

    #[test]
    fn test1() {
        assert_eq!(get_total_flashes(INPUT, 100), 1656)
    }

    #[test]
    fn test2() {
        assert_eq!(get_sync_step(INPUT, 300), 195)
    }

}