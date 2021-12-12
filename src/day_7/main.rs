fn main() {
    let input = include_str!("input.txt");

    println!("Min fuel is {}",get_min_fuel(input));
    println!("Min fuel rated is {}",get_min_fuel_rated(input));
}


fn get_min_fuel(input: &str) -> usize {
    let mut pos_array = Vec::new();
    for val_str in input.replace('\n', "").split(',') {
        pos_array.push(val_str.parse::<usize>().expect("Unable to parse"));
    }

    pos_array.sort_unstable();

    let median = 
    if pos_array.len() % 2 == 0 {
        //Even
        let pos_idx_h = pos_array.len()/2;
        let pos_idx_l = pos_idx_h - 1;
        (pos_array[pos_idx_h] + pos_array[pos_idx_l])/2
    } else {
        let pos_idx = pos_array.len()/2;
        pos_array[pos_idx]
    };

    println!("Median is {}",median);

    let mut fuel = 0;
    for val in pos_array {
        fuel += if val < median { median - val } else { val - median };
    }

    fuel
}


fn get_min_fuel_rated(input: &str) -> usize {
    let mut pos_array = Vec::new();
    for val_str in input.replace('\n', "").split(',') {
        pos_array.push(val_str.parse::<usize>().expect("Unable to parse"));
    }

    pos_array.sort_unstable();

    let mut sum = 0;
    for val in &pos_array {
        sum += *val;
    }
    let mean = sum / pos_array.len();

    println!("Mean is {}",mean);

    let fuel_l = fuel_used(&pos_array, mean-1);
    let fuel = fuel_used(&pos_array, mean);
    let fuel_h = fuel_used(&pos_array, mean+1);

    println!("{}: {}",mean-1,fuel_l);
    println!("{}: {}",mean,fuel);
    println!("{}: {}",mean+1,fuel_h);

    fuel_l.min(fuel.min(fuel_h))
}

fn fuel_used(input: &[usize], pos: usize) -> usize {
    let mut fuel = 0;
    for val in input {
        let dist = if *val < pos { pos - *val } else { *val - pos };
        fuel += dist * (dist + 1) / 2;
    }
    fuel
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
    "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test1() {
        assert_eq!(get_min_fuel(INPUT), 37);
    }

    #[test]
    fn test2() {
        assert_eq!(get_min_fuel_rated(INPUT), 168);
    }
}