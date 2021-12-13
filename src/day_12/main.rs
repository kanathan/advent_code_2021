use std::collections::HashMap;
use std::collections::HashSet;


fn main() {
    let input = include_str!("input.txt");

    println!("Total paths: {}", get_path_count(input, false));
    println!("Total paths with 2x small cave: {}", get_path_count(input, true));
}


fn get_path_count(input: &str, twice_visit_small_cave: bool) -> usize {
    let connections = parse_connections(input);
    let paths = get_paths(&connections, twice_visit_small_cave);
    paths.len()
}


fn get_paths(connections: &HashMap<String,HashSet<String>>, twice_visit_small_cave: bool) -> Vec<String> {
    let mut small_cave_entries: HashSet<String> = HashSet::new();
    let large_cave_entries: HashMap<String, usize> = HashMap::new();
    let has_visited_small_cave_twice = !twice_visit_small_cave;

    fn path_to_end(
        connections: &HashMap<String,HashSet<String>>,
        start: &String,
        cur_path: String,
        small_cave_entries: HashSet<String>,
        large_cave_entries: HashMap<String, usize>,
        has_visited_small_cave_twice: bool,
    ) -> Vec<String> {
        let mut paths = Vec::new();
        for connection in connections.get(start).unwrap() {
            if connection == "end" {
                paths.push(format!("{},end",cur_path));
            }
            else if connection == "start" {
                continue
            }
            else if connection.chars().all(|c| c.is_lowercase()) && (!small_cave_entries.contains(connection) || !has_visited_small_cave_twice) {
                let mut new_small_cave_entries = small_cave_entries.clone();
                let new_path = format!("{},{}",cur_path,connection);
                if small_cave_entries.contains(connection) {
                    paths.append(&mut path_to_end(connections, connection, new_path, new_small_cave_entries, large_cave_entries.clone(), true));
                }
                else {
                    new_small_cave_entries.insert(connection.clone());
                    paths.append(&mut path_to_end(connections, connection, new_path, new_small_cave_entries, large_cave_entries.clone(), has_visited_small_cave_twice));
                }
            }
            else if connection.chars().all(|c| c.is_uppercase()) {
                let mut new_large_cave_entries = large_cave_entries.clone();
                *new_large_cave_entries.entry(connection.clone()).or_insert(0) += 1;
                let new_path = format!("{},{}",cur_path,connection);
                paths.append(&mut path_to_end(connections, connection, new_path, small_cave_entries.clone(), new_large_cave_entries, has_visited_small_cave_twice));
            }
        }

        return paths
    }

    small_cave_entries.insert("start".to_string());
    path_to_end(connections, &"start".to_string(), "start".to_string(), small_cave_entries, large_cave_entries, has_visited_small_cave_twice)
}


fn parse_connections(input: &str) -> HashMap<String,HashSet<String>> {
    let mut connections = HashMap::new();

    for line in input.lines() {
        let (a,b) = line.split_once('-').unwrap();
        connections.entry(a.to_string()).or_insert(HashSet::new()).insert(b.to_string());
        connections.entry(b.to_string()).or_insert(HashSet::new()).insert(a.to_string());
    }
    connections
}



#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = 
    "start-A\n\
    start-b\n\
    A-c\n\
    A-b\n\
    b-d\n\
    A-end\n\
    b-end";

    const INPUT_2: &str =
    "dc-end\n\
    HN-start\n\
    start-kj\n\
    dc-start\n\
    dc-HN\n\
    LN-dc\n\
    HN-end\n\
    kj-sa\n\
    kj-HN\n\
    kj-dc";

    const INPUT_3: &str =
    "fs-end\n\
    he-DX\n\
    fs-he\n\
    start-DX\n\
    pj-DX\n\
    end-zg\n\
    zg-sl\n\
    zg-pj\n\
    pj-he\n\
    RW-he\n\
    fs-DX\n\
    pj-RW\n\
    zg-RW\n\
    start-pj\n\
    he-WI\n\
    zg-he\n\
    pj-fs\n\
    start-RW";

    #[test]
    fn test1() {
        assert_eq!(get_path_count(INPUT_1, false), 10);
        assert_eq!(get_path_count(INPUT_2, false), 19);
        assert_eq!(get_path_count(INPUT_3, false), 226);
    }

    #[test]
    fn test2() {
        assert_eq!(get_path_count(INPUT_1, true), 36);
        assert_eq!(get_path_count(INPUT_2, true), 103);
        assert_eq!(get_path_count(INPUT_3, true), 3509);
    }

}