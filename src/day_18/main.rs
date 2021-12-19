use regex::Regex;
use std::collections::HashSet;
use itertools::Itertools;
use lazy_static::lazy_static;

fn main() {
    let input = include_str!("input");

    let mut store = Vec::new();
    let mut lines = input.lines();
    let mut parent_idx = parse_string(lines.next().unwrap(), &mut store);
    for line in lines {
        parent_idx = add(parent_idx, line, &mut store);
        loop {
            if explode(parent_idx, &mut store) { continue }
            if split(parent_idx, &mut store) { continue }
            break;
        }
    }
    println!("{}",get_magnitude(parent_idx, &store));


    let max_mag = input
        .lines()
        .permutations(2)
        .map(|perm| {
            let mut store = Vec::new();
            let mut parent_idx = parse_string(perm[0], &mut store);
            parent_idx = add(parent_idx, perm[1], &mut store);
            loop {
                if explode(parent_idx, &mut store) { continue }
                if split(parent_idx, &mut store) { continue }
                break;
            }
            get_magnitude(parent_idx, &store)
        })
        .max().unwrap();
    println!("{}",max_mag);


}


fn add(parent_idx: PairIdx, str_b: &str, store: &mut Vec<NestPair>) -> PairIdx {
    let idx_b = parse_string(str_b, store);
    let new_pair = NestPair {left: NestValue::Pair(parent_idx), right: NestValue::Pair(idx_b)};
    let new_idx = store.len();
    store.push(new_pair);
    new_idx
}


fn explode(parent_idx: PairIdx, store: &mut Vec<NestPair>) -> bool {
    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    stack.push(parent_idx);
    while !stack.is_empty() {
        if let NestValue::Pair(idx) = store[*stack.last().unwrap()].left {
            if visited.insert(idx) {
                stack.push(idx);
                continue;
            }
        }
        if let NestValue::Pair(idx) = store[*stack.last().unwrap()].right {
            if visited.insert(idx) {
                stack.push(idx);
                continue;
            }
        }
        if stack.len() > 4 {
            break;
        }
        stack.pop();
    }

    if stack.len() <= 4 {
        return false
    }

    let explode_idx = stack.pop().unwrap();

    let idx_val_list = get_values(parent_idx, store);

    // Explode left
    let mut idx_val_list_iter_rev = idx_val_list.iter().rev();
    idx_val_list_iter_rev.find(|(idx, _, _)| *idx == explode_idx);
    if let Some((idx, side, val)) = idx_val_list_iter_rev.find(|(idx, _, _)| *idx != explode_idx) {
        if let NestValue::Val(exp_val) = store[explode_idx].left {
            if *side == 'l' { store[*idx].left = NestValue::Val(*val + exp_val) }
            else { store[*idx].right = NestValue::Val(*val + exp_val) }
        }
    }

    // Explode right
    let mut idx_val_list_iter = idx_val_list.iter();
    idx_val_list_iter.find(|(idx, _, _)| *idx == explode_idx);
    if let Some((idx, side, val)) = idx_val_list_iter.find(|(idx, _, _)| *idx != explode_idx) {
        if let NestValue::Val(exp_val) = store[explode_idx].right {
            if *side == 'l' { store[*idx].left = NestValue::Val(*val + exp_val) }
            else { store[*idx].right = NestValue::Val(*val + exp_val) }
        }
    }

    // Set original to zero
    let exp_parent = stack.pop().unwrap();
    if NestValue::Pair(explode_idx) == store[exp_parent].left {
        store[exp_parent].left = NestValue::Val(0);
    } else {
        store[exp_parent].right = NestValue::Val(0);
    }

    true
}

fn split(parent_idx: PairIdx, store: &mut Vec<NestPair>) -> bool {
    match store[parent_idx].left {
        NestValue::Val(val) => {
            if val > 9 {
                let div_down = val / 2;
                let new_pair = NestPair {left: NestValue::Val(div_down), right: NestValue::Val(val-div_down)};
                let new_idx = store.len();
                store.push(new_pair);
                store[parent_idx].left = NestValue::Pair(new_idx);
                return true
            }
        },
        NestValue::Pair(idx) => { if split(idx, store) { return true } },
    }
    match store[parent_idx].right {
        NestValue::Val(val) => {
            if val > 9 {
                let div_down = val / 2;
                let new_pair = NestPair {left: NestValue::Val(div_down), right: NestValue::Val(val-div_down)};
                let new_idx = store.len();
                store.push(new_pair);
                store[parent_idx].right = NestValue::Pair(new_idx);
                return true
            }
        },
        NestValue::Pair(idx) => { if split(idx, store) { return true } },
    }

    false
}

#[allow(dead_code)]
fn to_string(idx: PairIdx, store: &Vec<NestPair>) -> String {
    let left_str = match store[idx].left {
        NestValue::Val(val) => format!("{}",val),
        NestValue::Pair(idx) => to_string(idx, store),
    };
    let right_str = match store[idx].right {
        NestValue::Val(val) => format!("{}",val),
        NestValue::Pair(idx) => to_string(idx, store),
    };
    format!("[{},{}]",left_str,right_str)
}

fn get_values(idx: PairIdx, store: &Vec<NestPair>) -> Vec<(PairIdx,char,usize)> {
    let mut output = Vec::new();
    match store[idx].left {
        NestValue::Val(val) => output.push((idx, 'l', val)),
        NestValue::Pair(idx) => output.append(&mut get_values(idx, store)),
    };
    match store[idx].right {
        NestValue::Val(val) => output.push((idx, 'r', val)),
        NestValue::Pair(idx) => output.append(&mut get_values(idx, store)),
    };
    output
}

fn get_magnitude(idx: PairIdx, store: &Vec<NestPair>) -> usize {
    let mut output = 0;
    match store[idx].left {
        NestValue::Val(val) => output += 3 * val,
        NestValue::Pair(idx) => output += 3 * get_magnitude(idx, store),
    };
    match store[idx].right {
        NestValue::Val(val) => output += 2 * val,
        NestValue::Pair(idx) => output += 2 * get_magnitude(idx, store),
    };
    output
}


fn parse_string(input: &str, store: &mut Vec<NestPair>) -> PairIdx {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d+$").unwrap();
    }


    let mut comma_pos = None;
    let mut bracket_count = 0;
    for (pos, c) in input.chars().enumerate() {
        match c {
            '[' => bracket_count += 1,
            ']' => bracket_count -= 1,
            ',' => {if bracket_count == 1 { comma_pos = Some(pos) }},
            _ => (),
        }
        if comma_pos.is_some() { break }
    }
    let comma_pos = comma_pos.unwrap();
    let left_str = &input[1..comma_pos];
    let right_str = &input[comma_pos+1..input.len()-1];

    let left = if RE.is_match(left_str) {
        NestValue::Val(left_str.parse::<usize>().unwrap())
    } else {
        NestValue::Pair(parse_string(left_str, store))
    };

    let right = if RE.is_match(right_str) {
        NestValue::Val(right_str.parse::<usize>().unwrap())
    } else {
        NestValue::Pair(parse_string(right_str, store))
    };

    let pair = NestPair { left, right };
    let idx = store.len();
    store.push(pair);
    idx
}

type PairIdx = usize;

#[derive(Copy, Clone, Debug, PartialEq)]
enum NestValue {
    Val(usize),
    Pair(usize),
}

#[derive(Copy, Clone, Debug)]
struct NestPair {
    left: NestValue,
    right: NestValue,
}


#[cfg(test)]
mod test {
    use super::*;

    //const INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test1() {
        let mut store = Vec::new();
        let parent_idx = parse_string("[[[[4,3],4],4],[7,[[8,4],9]]]", &mut store);
        dbg!(get_values(parent_idx, &store));
    }

    #[test]
    fn test_magnitude() {
        let input = 
        "[[1,2],[[3,4],5]]\n\
        [[[[0,7],4],[[7,8],[6,0]]],[8,1]]\n\
        [[[[1,1],[2,2]],[3,3]],[4,4]]\n\
        [[[[3,0],[5,3]],[4,4]],[5,5]]\n\
        [[[[5,0],[7,4]],[5,5]],[6,6]]\n\
        [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";

        let truth: [usize; 6] = [143, 1384, 445, 791, 1137, 3488];

        for (line, val) in input.lines().zip(truth.iter()) {
            let mut store = Vec::new();
            let parent_idx = parse_string(line, &mut store);
            assert_eq!(get_magnitude(parent_idx, &store), *val);
        }

    }

    #[test]
    fn test_add() {
        let mut store = Vec::new();
        let mut parent_idx = parse_string("[[[[4,3],4],4],[7,[[8,4],9]]]", &mut store);
        parent_idx = add(parent_idx, "[1,1]", &mut store);
        loop {
            if explode(parent_idx, &mut store) { continue }
            if split(parent_idx, &mut store) { continue }
            break;
        }
        assert_eq!(to_string(parent_idx, &store), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_big_add() {
        let input = 
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n\
        [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n\
        [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]\n\
        [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]\n\
        [7,[5,[[3,8],[1,4]]]]\n\
        [[2,[2,2]],[8,[8,1]]]\n\
        [2,9]\n\
        [1,[[[9,3],9],[[9,0],[0,7]]]]\n\
        [[[5,[7,4]],7],1]\n\
        [[[[4,2],2],6],[8,7]]";

        let mut store = Vec::new();

        let mut lines = input.lines();
        let mut parent_idx = parse_string(lines.next().unwrap(), &mut store);
        for line in lines {
            parent_idx = add(parent_idx, line, &mut store);
            loop {
                if explode(parent_idx, &mut store) { continue }
                if split(parent_idx, &mut store) { continue }
                break;
            }
        }
        assert_eq!(to_string(parent_idx, &store), "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    }

    #[test]
    fn test_explode() {
        assert_eq!(explode_once("[[[[[9,8],1],2],3],4]"),"[[[[0,9],2],3],4]");
        assert_eq!(explode_once("[7,[6,[5,[4,[3,2]]]]]"),"[7,[6,[5,[7,0]]]]");
        assert_eq!(explode_once("[[6,[5,[4,[3,2]]]],1]"),"[[6,[5,[7,0]]],3]");
        assert_eq!(explode_once("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"),"[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert_eq!(explode_once("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),"[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    fn explode_once(input: &str) -> String {
        let mut store = Vec::new();
        let parent_idx = parse_string(input, &mut store);
        explode(parent_idx, &mut store);
        to_string(parent_idx, &store)
    }

    #[test]
    fn test_split() {
        assert_eq!(split_once("[[[[0,7],4],[15,[0,13]]],[1,1]]"),"[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
        assert_eq!(split_once("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"),"[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
    }

    fn split_once(input: &str) -> String {
        let mut store = Vec::new();
        let parent_idx = parse_string(input, &mut store);
        split(parent_idx, &mut store);
        to_string(parent_idx, &store)
    }
}