use std::collections::HashMap;
use lazy_static::lazy_static;

fn main() {
    let input = include_str!("input.txt");

    println!("Error score is {}",get_error_score(input));
    println!("Completion score is {}",get_completion_score(input));
}

fn get_error_score(input: &str) -> usize {
    let mut score = 0;
    for line in input.lines() {
        if let Err(invalid_char) = process_line(line) {
            score += match invalid_char {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("Invalid char {}",invalid_char)
            };
        }
    }
    score
}

fn get_completion_score(input: &str) -> usize {
    let mut tot_scores = Vec::new();

    for line in input.lines() {
        if let Ok(char_stack) = process_line(line) {
            let mut line_score = 0;
            for cur_char in char_stack.iter().rev() {
                match cur_char {
                    '(' => {
                        //print!(")");
                        line_score *= 5;
                        line_score += 1;
                    },
                    '[' => {
                        //print!("]");
                        line_score *= 5;
                        line_score += 2;
                    },
                    '{' => {
                        //print!("}}");
                        line_score *= 5;
                        line_score += 3;
                    },
                    '<' => {
                        //print!(">");
                        line_score *= 5;
                        line_score += 4;
                    },
                    _ => panic!("Invalid char {}",cur_char)
                }
            }
            //println!(" - {} total points",line_score);
            tot_scores.push(line_score);
        }
    }

    tot_scores.sort();
    *tot_scores.get(tot_scores.len()/2).unwrap()
}

fn process_line(line: &str) -> Result<Vec<char>, char> {
    lazy_static! {
        static ref DELIMITER_MAP: HashMap<char, char> = HashMap::from([
            (')', '('),
            (']', '['),
            ('}', '{'),
            ('>', '<'),
        ]);
    }

    let mut char_stack = Vec::new();

    for cur_char in line.chars() {
        match cur_char {
            '(' | '[' | '{' | '<' => char_stack.push(cur_char),
            ')' | ']' | '}' | '>' => {
                let open_char = *DELIMITER_MAP.get(&cur_char).unwrap();
                let last_delim = char_stack.pop();
                if last_delim.is_none() || last_delim.unwrap() != open_char {
                    return Err(cur_char)
                }
            }
            _ => panic!("Invalid character {}",cur_char)
        }
    }

    Ok(char_stack)
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
    "[({(<(())[]>[[{[]{<()<>>\n\
    [(()[<>])]({[<{<<[]>>(\n\
    {([(<{}[<>[]}>{[]{[(<()>\n\
    (((({<>}<{<{<>}{[]{[]{}\n\
    [[<[([]))<([[{}[[()]]]\n\
    [{[{({}]{}}([{[{{{}}([]\n\
    {<[[]]>}<{[{[{[]{()[[[]\n\
    [<(<(<(<{}))><([]([]()\n\
    <{([([[(<>()){}]>(<<{{\n\
    <{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test1() {
        assert_eq!(get_error_score(INPUT), 26397)
    }

    #[test]
    fn test2() {
        assert_eq!(get_completion_score(INPUT), 288957)
    }

}