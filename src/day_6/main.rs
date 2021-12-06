

fn main() {
    let input = include_str!("input.txt");
    let input = input.replace("\n", "");

    println!("Fish count @ 80: {}",get_fish_count(&input, 80));
    println!("Fish count @ 256: {}",get_fish_count(&input, 256));
}


fn get_fish_count(input: &str, days: usize) -> usize {
    let mut fish_array = vec![0; 9];
    let mut cur_idx = 0;

    for val_str in input.split(",") {
        let idx = val_str.parse::<usize>().expect("Invalid input");
        *fish_array.get_mut(idx).expect(&format!("Invalid day {}", idx)) += 1;
    }

    for day in 0..days {
        let spawning_fish = fish_array[cur_idx];
        fish_array[cur_idx] = 0;
        cur_idx = (cur_idx + 1) % 9;
        fish_array[(cur_idx + 6) % 9] += spawning_fish;
        fish_array[(cur_idx + 8) % 9] += spawning_fish;
        println!("Day {}: Added {} fish",day+1,spawning_fish);
    }

    let mut fish_count = 0;
    for fish in fish_array.iter() {
        fish_count += *fish;
    }
    fish_count
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
        "3,4,3,1,2";


    #[test]
    fn test1() {
        assert_eq!(get_fish_count(INPUT, 18), 26);
        assert_eq!(get_fish_count(INPUT, 80), 5934);
        assert_eq!(get_fish_count(INPUT, 256), 26984457539);
    }
}