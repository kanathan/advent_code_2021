
fn main() {
    let input = include_str!("input.txt");
    
    let val = get_gamma_epsilon(input);
    println!("Final gamma/epsilon: {} x {} = {}",val.0, val.1, val.0 * val.1);

    let val = get_life_support(input);
    println!("Final ox/co2: {} x {} = {}",val.0, val.1, val.0 * val.1);
}

fn get_gamma_epsilon(input: &str) -> (i64, i64) {
    let mut counts = Vec::new();
    let mut size: usize = 0;

    for _ in 0..input.lines().next().unwrap().len() {
        counts.push(0);
    }

    for line in input.lines() {
        for (char, count) in line.chars().zip(counts.iter_mut()) {
            if char == '1' { *count += 1 };
        }
        size += 1;
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for count in counts {
        gamma <<= 1;
        epsilon <<= 1;
        if count * 2 >= size { gamma += 1 }
        else { epsilon += 1 }
    }

    (gamma, epsilon)
}

fn get_life_support(input: &str) -> (i64, i64) {
    let mut ox_values = Vec::new();
    for line in input.lines() {
        ox_values.push(line)
    }
    let mut co2_values = ox_values.clone();

    let bit_length = ox_values.get(0).unwrap().len();
    for bit_pos in 0..bit_length {
        if ox_values.len() > 1 {
            let mut count = 0;
            for bit_str in &ox_values {
                if bit_str.chars().nth(bit_pos).unwrap() == '1' {
                    count += 1;
                }
            }
            if count * 2 >= ox_values.len() {
                ox_values.retain(|&bit_str| bit_str.chars().nth(bit_pos).unwrap() == '1')
            }
            else {
                ox_values.retain(|&bit_str| bit_str.chars().nth(bit_pos).unwrap() == '0')
            }
        }
        if co2_values.len() > 1 {
            let mut count = 0;
            for bit_str in &co2_values {
                if bit_str.chars().nth(bit_pos).unwrap() == '1' {
                    count += 1;
                }
            }
            if count * 2 < co2_values.len() {
                co2_values.retain(|&bit_str| bit_str.chars().nth(bit_pos).unwrap() == '1')
            }
            else {
                co2_values.retain(|&bit_str| bit_str.chars().nth(bit_pos).unwrap() == '0')
            }
        }
        if ox_values.len() == 1 && co2_values.len() == 1 { break }
    }
    
    let mut ox_rating = 0;
    for char in ox_values.first().unwrap().chars() {
        ox_rating <<= 1;
        if char == '1' { ox_rating += 1 }
    }

    let mut co2_rating = 0;
    for char in co2_values.first().unwrap().chars() {
        co2_rating <<= 1;
        if char == '1' { co2_rating += 1 }
    }

    (ox_rating, co2_rating)
}





#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
        "00100\n\
        11110\n\
        10110\n\
        10111\n\
        10101\n\
        01111\n\
        00111\n\
        11100\n\
        10000\n\
        11001\n\
        00010\n\
        01010";

    #[test]
    fn test1() {
        assert_eq!(get_gamma_epsilon(INPUT), (22, 9));
    }

    #[test]
    fn test2() {
        assert_eq!(get_life_support(INPUT), (23, 10));
    }
}