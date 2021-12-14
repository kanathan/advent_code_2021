use std::collections::HashMap;

fn main() {
    let input = include_str!("input");

    println!("{}",get_min_max_opt(input, 10));
    println!("{}",get_min_max_opt(input, 40));
}


fn get_min_max_opt(input: &str, steps: usize) -> usize {
    let (template, rules) = parse_input_count(input);
    let mut pair_count = HashMap::new();

    for idx in 0..template.len()-1 {
        *pair_count.entry(template[idx..idx+2].to_string()).or_insert(0) += 1;
    }

    for _ in 0..steps {
        let mut new_pair_count = HashMap::new();
        for (key, val) in pair_count.iter() {
            let pair = format!("{}{}",key.chars().nth(0).unwrap(),rules[key]);
            *new_pair_count.entry(pair).or_insert(0) += val;
            let pair = format!("{}{}",rules[key],key.chars().nth(1).unwrap());
            *new_pair_count.entry(pair).or_insert(0) += val;
        }
        pair_count = new_pair_count;
    }

    let mut char_counts = HashMap::new();
    for (key, val) in pair_count.iter() {
        *char_counts.entry(key.chars().nth(0).unwrap()).or_insert(0) += val;
    }
    *char_counts.entry(template.chars().last().unwrap()).or_insert(0) += 1;

    let mut count_vec: Vec<_> = char_counts.iter().collect();
    count_vec.sort_by(|a, b| a.1.cmp(b.1));

    *count_vec.iter().last().unwrap().1 - *count_vec.iter().next().unwrap().1
}


fn parse_input_count(input: &str) -> (String, HashMap<String, String>) {
    let mut lines = input.lines();
    let template = lines.next().unwrap().to_string();
    let mut rules = HashMap::new();

    lines.next();
    for line in lines {
        let (k, v) = line.split_once(" -> ").expect(&format!("Invalid input: {}",line));
        rules.insert(k.to_string(), v.to_string());
    }

    (template, rules)
}



#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
    "NNCB\n\
    \n\
    CH -> B\n\
    HH -> N\n\
    CB -> H\n\
    NH -> C\n\
    HB -> C\n\
    HC -> B\n\
    HN -> C\n\
    NN -> C\n\
    BH -> H\n\
    NC -> B\n\
    NB -> B\n\
    BN -> B\n\
    BB -> N\n\
    BC -> B\n\
    CC -> N\n\
    CN -> C";

    #[test]
    fn test1() {
        assert_eq!(get_min_max_opt(INPUT, 10), 1588);
    }

    #[test]
    fn test2() {
        assert_eq!(get_min_max_opt(INPUT, 40), 2188189693529);
    }
}