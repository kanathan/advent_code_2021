use std::collections::VecDeque;

fn main() -> Result<(), std::io::Error> {
    let input = include_str!("input.txt");

    // Part 1
    println!("Per item increase count is {}",per_line_inc_count(input));

    // Part 2
    println!("Moving window increase count is {}",per_window_inc_count(3, input));

    Ok(())
}

fn per_line_inc_count(input: &str) -> i64 {
    let mut lines = input.lines();
    
    let mut item_inc_count = 0;
    let mut last_value: i64 = lines.next().expect("No data in file").parse().expect("Invalid input");

    for line in lines {
        let cur_value: i64 = line.parse().expect("Invalid input");
        if cur_value > last_value { item_inc_count += 1 };
        last_value = cur_value;
    }

    item_inc_count
}

fn per_window_inc_count(window_size: usize, input: &str) -> i64 {
    let mut lines = input.lines();

    let mut item_window_count = 0;
    let mut cur_sum: i64 = 0;
    let mut window = VecDeque::new();

    for _ in 0..window_size {
        window.push_back(lines.next().expect("Not enough data in file").parse().expect("Invalid input"));
        cur_sum += window.back().unwrap();
    }

    for line in lines {
        let last_sum = cur_sum;
        cur_sum -= window.pop_front().unwrap();
        window.push_back(line.parse().expect("Invalid input"));
        cur_sum += window.back().unwrap();
        if cur_sum > last_sum { item_window_count += 1 };
    }

    item_window_count
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
        "199\n\
        200\n\
        208\n\
        210\n\
        200\n\
        207\n\
        240\n\
        269\n\
        260\n\
        263";

    #[test]
    fn test1() {
        assert_eq!(per_line_inc_count(INPUT), 7);
    }

    #[test]
    fn test2() {
        assert_eq!(per_window_inc_count(3, INPUT), 5);
    }
}