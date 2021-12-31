use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Add;
use std::ops::Sub;
use itertools::Itertools;

fn main() {
    let input = include_str!("input");

    let (scanners, beacons) = get_scanners_beacons(input, 0);
    println!("{}",beacons.len());
    let max_dist = scanners.iter()
        .combinations(2)
        .map(|v| (v[0].1, v[1].1))
        .map(|(a, b)| (a.x-b.x).abs() + (a.y-b.y).abs() + (a.z-b.z).abs())
        .max().unwrap();
    println!("{}",max_dist);
}


fn get_scanners_beacons(input: &str, origin: usize) -> (HashMap<usize, Coords>, Vec<Coords>) {
    let mut coord_sets: HashMap<(i64, i64, i64), Vec<(Coords, Coords)>> = HashMap::new();
    let mut unique_coords = HashSet::new();
    let mut scanner_info = HashMap::new();

    let coord_map = parse_input(input);
    let mut completed_scanners = HashSet::new();

    for coord_set in coord_map.get(&origin).unwrap().iter().combinations(2) {
        let coord_tuple = (
            coord_set[0].length(coord_set[1]),
            coord_set[0].min(coord_set[1]),
            coord_set[0].max(coord_set[1]),
        );

        coord_sets.entry(coord_tuple).or_insert(vec![]).push((*coord_set[0], *coord_set[1]));

        unique_coords.insert(coord_set[0].clone());
        unique_coords.insert(coord_set[1].clone());
    }
    scanner_info.insert(origin, Coords {x: 0, y: 0, z: 0});
    completed_scanners.insert(origin);

    while completed_scanners.len() < coord_map.len() {
        let mut scanner_score: HashMap<(usize, Matrix3, Coords), usize> = HashMap::new();
        for (scanner_id, coord_vec) in coord_map.iter() {
            if completed_scanners.contains(scanner_id) { continue }
    
            for (c1, c2) in coord_vec.iter().combinations(2).map(|c| (c[0], c[1])) {
                let coord_tuple = (
                    c1.length(c2),
                    c1.min(c2),
                    c1.max(c2),
                );
    
                if let Some(origin_c_vec) = coord_sets.get(&coord_tuple) {
                    for rot in ROTATIONS {
                        let cur_c1_rot = c1.rotate(rot);
                        let cur_c2_rot = c2.rotate(rot);
                        for (origin_c1, origin_c2) in origin_c_vec {
                            if cur_c1_rot - *origin_c1 == cur_c2_rot - *origin_c2 {
                                *scanner_score.entry((*scanner_id, rot, *origin_c1 - cur_c1_rot)).or_insert(0) += 1
                            }
                            else if cur_c1_rot - *origin_c2 == cur_c2_rot - *origin_c1 {
                                *scanner_score.entry((*scanner_id, rot, *origin_c2 - cur_c1_rot)).or_insert(0) += 1
                            }
                        }
                    }
                }
            }            
        }

        let ((scanner_id, rot, offset), score) = scanner_score.iter().max_by(|a, b| {
            a.1.cmp(&b.1)
        }).unwrap();

        println!("Adding beacons from scanner {} after finding {} matches",scanner_id, score);

        let rot_coords = rotate_all(coord_map.get(&scanner_id).unwrap(), *rot);
        for coord_set in rot_coords.iter().combinations(2) {
            let coord_tuple = (
                coord_set[0].length(coord_set[1]),
                coord_set[0].min(coord_set[1]),
                coord_set[0].max(coord_set[1]),
            );

            let c1 = *offset + *coord_set[0];
            let c2 = *offset + *coord_set[1];
    
            coord_sets.entry(coord_tuple).or_insert(vec![]).push((c1, c2));
    
            unique_coords.insert(c1.clone());
            unique_coords.insert(c2.clone());
        }
        scanner_info.insert(*scanner_id, *offset);
        completed_scanners.insert(*scanner_id);
    }

    
    (scanner_info,unique_coords.into_iter().collect())
}


fn rotate_all(coords: &Vec<Coords>, rotation: Matrix3) -> Vec<Coords> {
    coords.iter()
    .map(|c| c.rotate(rotation))
    .collect()
}


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Coords {
    x: i64,
    y: i64,
    z: i64,
}

impl Coords {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    fn length(&self, other: &Coords) -> i64 {
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;
        let z_diff = other.z - self.z;
        ((x_diff.pow(2)+y_diff.pow(2)+z_diff.pow(2)) as f32).sqrt() as i64
    }

    fn min(&self, other: &Coords) -> i64 {
        let x_diff = (other.x - self.x).abs();
        let y_diff = (other.y - self.y).abs();
        let z_diff = (other.z - self.z).abs();
        x_diff.min(y_diff.min(z_diff))
    }

    fn max(&self, other: &Coords) -> i64 {
        let x_diff = (other.x - self.x).abs();
        let y_diff = (other.y - self.y).abs();
        let z_diff = (other.z - self.z).abs();
        x_diff.max(y_diff.max(z_diff))
    }

    fn rotate(&self, rotation: Matrix3) -> Self {
        Self {
            x: self.x * rotation.0.0 + self.y * rotation.0.1 + self.z * rotation.0.2,
            y: self.x * rotation.1.0 + self.y * rotation.1.1 + self.z * rotation.1.2,
            z: self.x * rotation.2.0 + self.y * rotation.2.1 + self.z * rotation.2.2,
        }
    }
}

impl Sub for Coords {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

type Matrix3 = ((i64, i64, i64), (i64, i64, i64), (i64, i64, i64));

const ROT01: Matrix3 =
    ((  1,  0,  0),
     (  0,  1,  0),
     (  0,  0,  1));
 
const ROT02: Matrix3 =
    ((  1,  0,  0),
     (  0,  0, -1),
     (  0,  1,  0));
 
const ROT03: Matrix3 =
    ((  1,  0,  0),
     (  0, -1,  0),
     (  0,  0, -1));
 
const ROT04: Matrix3 =
    ((  1,  0,  0),
     (  0,  0,  1),
     (  0, -1,  0));
 
const ROT05: Matrix3 =
    ((  0, -1,  0),
     (  1,  0,  0),
     (  0,  0,  1));
 
const ROT06: Matrix3 =
    ((  0,  0,  1),
     (  1,  0,  0),
     (  0,  1,  0));
 
const ROT07: Matrix3 =
    ((  0,  1,  0),
     (  1,  0,  0),
     (  0,  0, -1));
 
const ROT08: Matrix3 =
    ((  0,  0, -1),
     (  1,  0,  0),
     (  0, -1,  0));
 
const ROT09: Matrix3 =
    (( -1,  0,  0),
     (  0, -1,  0),
     (  0,  0,  1));
 
const ROT10: Matrix3 =
    (( -1,  0,  0),
     (  0,  0, -1),
     (  0, -1,  0));
 
const ROT11: Matrix3 =
    (( -1,  0,  0),
     (  0,  1,  0),
     (  0,  0, -1));
 
const ROT12: Matrix3 =
    (( -1,  0,  0),
     (  0,  0,  1),
     (  0,  1,  0));
 
const ROT13: Matrix3 =
    ((  0,  1,  0),
     ( -1,  0,  0),
     ( 0,   0,  1));
 
const ROT14: Matrix3 =
    ((  0,  0,  1),
     ( -1,  0,  0),
     (  0, -1,  0));
 
const ROT15: Matrix3 =
    ((  0, -1,  0),
     ( -1,  0,  0),
     (  0,  0, -1));
 
const ROT16: Matrix3 =
    ((  0,  0, -1),
     ( -1,  0,  0),
     (  0,  1,  0));
 
const ROT17: Matrix3 =
    ((  0,  0, -1),
     (  0,  1,  0),
     (  1,  0,  0));
 
const ROT18: Matrix3 =
    ((  0,  1,  0),
     (  0,  0,  1),
     (  1,  0,  0));
 
const ROT19: Matrix3 =
    ((  0,  0,  1),
     (  0, -1,  0),
     (  1,  0,  0));
 
const ROT20: Matrix3 =
    ((  0, -1,  0),
     (  0,  0, -1),
     (  1,  0,  0));
 
const ROT21: Matrix3 =
    ((  0,  0, -1),
     (  0, -1,  0),
     ( -1,  0,  0));
 
const ROT22: Matrix3 =
    ((  0, -1,  0),
     (  0,  0,  1),
     ( -1,  0,  0));
 
const ROT23: Matrix3 =
    ((  0,  0,  1),
     (  0,  1,  0),
     ( -1,  0,  0));
 
const ROT24: Matrix3 =
    ((  0,  1,  0),
     (  0,  0, -1),
     ( -1,  0,  0));

const ROTATIONS: [Matrix3; 24] = [
    ROT01, ROT02, ROT03, ROT04, ROT05, ROT06, ROT07, ROT08, ROT09, ROT10, ROT11, ROT12,
    ROT13, ROT14, ROT15, ROT16, ROT17, ROT18, ROT19, ROT20, ROT21, ROT22, ROT23, ROT24
];


fn parse_input(input: &str) -> HashMap<usize, Vec<Coords>> {
    let re = Regex::new(r"--- scanner (\d+) ---").unwrap();

    let mut lines = input.lines();
    let mut output: HashMap<usize, Vec<Coords>> = HashMap::new();

    while let Some(line) = lines.next() {
        let idx = re.captures(line).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
        let mut coord_vec = Vec::new();
        
        loop {
            if let Some(next_coord_str) = lines.next() {
                if next_coord_str.is_empty() { break }
                let coords: Vec<i64> = next_coord_str.splitn(3, ',').map(|s| s.parse::<i64>().unwrap()).collect();
                coord_vec.push(Coords::new(coords[0], coords[1], coords[2]));
            }
            else { break }
        }
        output.insert(idx, coord_vec);
    }

    output
}

/*fn get_transform_matrices() -> Vec<Rotation3<i64>> {
    /*let mut output = Vec::new();
    for a in (0..271).step_by(90) {
        let rot_a = Rotation3::from_axis_angle(&Vector3::x_axis(), (a as f32) * std::f32::consts::PI / 180.0);
        for b in (0..271).step_by(90) {
            let rot_b = Rotation3::from_axis_angle(&Vector3::y_axis(), (b as f32) * std::f32::consts::PI / 180.0);
            output.push(rot_a * rot_b);
        }
    }
    for a in (0..271).step_by(90) {
        let rot_a = Rotation3::from_axis_angle(&Vector3::x_axis(), (a as f32) * std::f32::consts::PI / 180.0);
        for b in [90, 270] {
            let rot_b = Rotation3::from_axis_angle(&Vector3::z_axis(), (b as f32) * std::f32::consts::PI / 180.0);
            output.push(rot_a * rot_b);
        }
    }*/

    vec![

    ]
}*/


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("test");

        //get_transform_matrices();

        let (scanners, beacons) = get_scanners_beacons(input, 0);
        println!("{}",beacons.len());
        println!("{:?}",scanners);
        let max_dist = scanners.iter()
            .combinations(2)
            .map(|v| (v[0].1, v[1].1))
            .map(|(a, b)| (a.x-b.x).abs() + (a.y-b.y).abs() + (a.z-b.z).abs())
            .max().unwrap();
        println!("{}",max_dist);
    }   

}