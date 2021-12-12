use lazy_static::lazy_static;
use regex::Regex;

use std::{collections::{HashSet, HashMap}, iter::FromIterator};

fn main() {
    let input = include_str!("input.txt");

    println!("Segment 1,4,7,8 count: {}",get_unique_segment_count(input));
    println!("Output total is {}",get_output_total(input));
}


fn get_unique_segment_count(input: &str) -> usize {
    lazy_static! {
        static ref RE_LINE: Regex = Regex::new(r"(.+) \| (.+)").unwrap();
        static ref RE_SEGMENT: Regex = Regex::new(r"(\w+)").unwrap();
    }

    let mut count = 0;

    for line in input.lines() {
        let collection = RE_LINE.captures(line).unwrap();
        let seg_output = collection.get(2).unwrap().as_str();
        for seg_match in RE_SEGMENT.find_iter(seg_output) {
            match seg_match.as_str().len() {
                2 | 4 | 3 | 7 => count += 1,
                _ => ()
            }
        }
    }

    count
}

fn get_output_total(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        total += get_output(line);
    }
    total
}

fn get_output(line: &str) -> usize {
    lazy_static! {
        static ref RE_LINE: Regex = Regex::new(r"(.+) \| (.+)").unwrap();
        static ref RE_SEGMENT: Regex = Regex::new(r"(\w+)").unwrap();
    }

    let collection = RE_LINE.captures(line).unwrap();
    let seg_input_str = collection.get(1).unwrap().as_str();
    let seg_output_str = collection.get(2).unwrap().as_str();

    let seg_inputs: Vec<String> = RE_SEGMENT.find_iter(seg_input_str).map(|m| m.as_str().to_string()).collect();
    let seg_outputs: Vec<String> = RE_SEGMENT.find_iter(seg_output_str).map(|m| m.as_str().to_string()).collect();

    // Get known length segments
    let mut known_segments: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut unknown_segments: Vec<HashSet<char>> = Vec::new();
    for seg_input in seg_inputs.iter() {
        match seg_input.len() {
            2 => { known_segments.insert(1, HashSet::from_iter(seg_input.chars())); },
            3 => { known_segments.insert(7, HashSet::from_iter(seg_input.chars())); },
            4 => { known_segments.insert(4, HashSet::from_iter(seg_input.chars())); },
            7 => { known_segments.insert(8, HashSet::from_iter(seg_input.chars())); },
            _ => { unknown_segments.push(HashSet::from_iter(seg_input.chars())) },
        }
    }

    for segment in unknown_segments {
        match segment.len() {
            5 => {
                if (segment.intersection(&known_segments[&1])).count() == 2 { known_segments.insert(3, segment); }
                else if (segment.intersection(&known_segments[&4])).count() == 2 { known_segments.insert(2, segment); }
                else { known_segments.insert(5, segment); }
            },
            6 => {
                if (segment.intersection(&known_segments[&1])).count() == 1 { known_segments.insert(6, segment); }
                else if (segment.intersection(&known_segments[&4])).count() == 4 { known_segments.insert(9, segment); }
                else { known_segments.insert(0, segment); }
            },
            _ => panic!("Unknown segment")
        }
    }

    let mut output = 0;
    for seg_output in seg_outputs.iter() {
        let segment: HashSet<char> = HashSet::from_iter(seg_output.chars());
        output *= 10;
        for (k,v) in known_segments.iter() {
            if *v == segment {
                output += k;
                break;
            }
        }
    }

    output
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test1() {
        assert_eq!(get_unique_segment_count(INPUT), 26);
    }

    #[test]
    fn test2() {
        let line = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(get_output(line), 5353);
    }

    #[test]
    fn test3() {
        assert_eq!(get_output_total(INPUT), 61229);
    }
}