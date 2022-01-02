use std::ops::RangeInclusive;
use ndarray::Array3;
use regex::Regex;
use std::collections::HashMap;


fn main() {
    let input = include_str!("input");

    let instructions = parse(input, Some((-50..=50,-50..=50,-50..=50)));
    println!("{}",get_count(&instructions));

    let instructions = parse(input, None);
    println!("{}",get_count(&instructions));
}

#[allow(dead_code)]
fn get_count_slow(mut grid: Array3<bool>, offsets: [i64; 3], instructions: &Vec<Instruction>) -> usize {
    let mut count = 0;

    for (step_idx, inst) in instructions.iter().enumerate() {
        println!("Performing step {} of {}: {:?}",step_idx+1,instructions.len(), inst);
        for x in inst.x_range.clone().map(|v| v+offsets[0]).filter(|v| *v >= 0).map(|v| v as usize) {
            if x >= grid.shape()[0] { break }
            for y in inst.y_range.clone().map(|v| v+offsets[1]).filter(|v| *v >= 0).map(|v| v as usize) {
                if y >= grid.shape()[1] { break }
                for z in inst.z_range.clone().map(|v| v+offsets[2]).filter(|v| *v >= 0).map(|v| v as usize) {
                    if z >= grid.shape()[2] { break }
                    if let Some(val_ref) = grid.get_mut([x,y,z]) {
                        if inst.ty && !*val_ref {count += 1}
                        else if !inst.ty && *val_ref { count -= 1}
                        *val_ref = inst.ty;
                    }
                }
            }
        }
    }

    count
}


fn get_count(instructions: &Vec<Instruction>) -> usize {
    let mut cube_map: HashMap<Cube,i64> = HashMap::new();

    for (step_idx, inst) in instructions.iter().enumerate() {
        println!("Performing step {} of {}: {:?}",step_idx+1,instructions.len(), inst);
        if inst.x_range.is_empty() || inst.y_range.is_empty() || inst.z_range.is_empty() { continue }

        let inst_cube = Cube {
            x_min: *inst.x_range.start(),
            x_max: *inst.x_range.end(),
            y_min: *inst.y_range.start(),
            y_max: *inst.y_range.end(),
            z_min: *inst.z_range.start(),
            z_max: *inst.z_range.end(),
        };

        let mut cube_map_updates = HashMap::new();

        for (cube, cnt) in cube_map.iter() {
            if let Some(new_cube) = cube.intersect(&inst_cube) {
                *cube_map_updates.entry(new_cube).or_insert(0) -= cnt;
            }
        }
        if inst.ty {
            *cube_map_updates.entry(inst_cube).or_insert(0) += 1;
        }
        
        for (cube, cnt) in cube_map_updates {
            *cube_map.entry(cube).or_insert(0) += cnt;
        }

        /*let sum = 
            cube_map.iter().map(|(c,v)| {
                ((c.x_max-c.x_min+1)*(c.y_max-c.y_min+1)*(c.z_max-c.z_min+1))*v
            }).sum::<i64>();
        println!("Cur sum: {}",sum);*/
    }

    cube_map.iter().map(|(c,v)| {
        ((c.x_max-c.x_min+1)*(c.y_max-c.y_min+1)*(c.z_max-c.z_min+1))*v
    }).sum::<i64>() as usize
}


fn parse(input: &str, range: Option<(RangeInclusive<i64>, RangeInclusive<i64>, RangeInclusive<i64>)>) -> Vec<Instruction> {
    let re = Regex::new(r"((?:on)|(?:off)) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();
    let mut instructions = Vec::new();

    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let mut x_min = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let mut x_max = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let mut y_min = captures.get(4).unwrap().as_str().parse::<i64>().unwrap();
        let mut y_max = captures.get(5).unwrap().as_str().parse::<i64>().unwrap();
        let mut z_min = captures.get(6).unwrap().as_str().parse::<i64>().unwrap();
        let mut z_max = captures.get(7).unwrap().as_str().parse::<i64>().unwrap();
        if let Some((x_range,y_range,z_range)) = range.as_ref() {
            x_min = x_min.max(*x_range.start());
            x_max = x_max.min(*x_range.end());
            y_min = y_min.max(*y_range.start());
            y_max = y_max.min(*y_range.end());
            z_min = z_min.max(*z_range.start());
            z_max = z_max.min(*z_range.end());
        }
        instructions.push(
            Instruction {
                ty: captures.get(1).unwrap().as_str() == "on",
                x_range: x_min..=x_max,
                y_range: y_min..=y_max,
                z_range: z_min..=z_max,
            }
        )
    }

    instructions
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Cube {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    z_min: i64,
    z_max: i64,
}

impl Cube {
    fn intersect(&self, other: &Self) -> Option<Self> {
        let x_min = self.x_min.max(other.x_min);
        let x_max = self.x_max.min(other.x_max);
        let y_min = self.y_min.max(other.y_min);
        let y_max = self.y_max.min(other.y_max);
        let z_min = self.z_min.max(other.z_min);
        let z_max = self.z_max.min(other.z_max);

        if x_min <= x_max && y_min <= y_max && z_min <= z_max {
            Some(Cube {
                x_min,
                x_max,
                y_min,
                y_max,
                z_min,
                z_max,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Instruction {
    ty: bool,
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
    z_range: RangeInclusive<i64>,
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "on x=10..12,y=10..12,z=10..12\n\
    on x=11..13,y=11..13,z=11..13\n\
    off x=9..11,y=9..11,z=9..11\n\
    on x=10..10,y=10..10,z=10..10";

    const INPUT2: &str = 
    "on x=-20..26,y=-36..17,z=-47..7\n\
    on x=-20..33,y=-21..23,z=-26..28\n\
    on x=-22..28,y=-29..23,z=-38..16\n\
    on x=-46..7,y=-6..46,z=-50..-1\n\
    on x=-49..1,y=-3..46,z=-24..28\n\
    on x=2..47,y=-22..22,z=-23..27\n\
    on x=-27..23,y=-28..26,z=-21..29\n\
    on x=-39..5,y=-6..47,z=-3..44\n\
    on x=-30..21,y=-8..43,z=-13..34\n\
    on x=-22..26,y=-27..20,z=-29..19\n\
    off x=-48..-32,y=26..41,z=-47..-37\n\
    on x=-12..35,y=6..50,z=-50..-2\n\
    off x=-48..-32,y=-32..-16,z=-15..-5\n\
    on x=-18..26,y=-33..15,z=-7..46\n\
    off x=-40..-22,y=-38..-28,z=23..41\n\
    on x=-16..35,y=-41..10,z=-47..6\n\
    off x=-32..-23,y=11..30,z=-14..3\n\
    on x=-49..-5,y=-3..45,z=-29..18\n\
    off x=18..30,y=-20..-8,z=-3..13\n\
    on x=-41..9,y=-7..43,z=-33..15\n\
    on x=-54112..-39298,y=-85059..-49293,z=-27449..7877\n\
    on x=967..23432,y=45373..81175,z=27513..53682";

    #[test]
    fn test1() {
        let instructions = parse(INPUT, None);
        let grid = Array3::from_elem([101,101,101], false);
        let offsets = [50,50,50];
        assert_eq!(get_count_slow(grid, offsets, &instructions), 39);

        let instructions = parse(INPUT2, None);
        let grid = Array3::from_elem([101,101,101], false);
        let offsets = [50,50,50];
        assert_eq!(get_count_slow(grid, offsets, &instructions), 590784);

        let instructions = parse(INPUT, Some((-50..=50,-50..=50,-50..=50)));
        let grid = Array3::from_elem([101,101,101], false);
        let offsets = [50,50,50];
        assert_eq!(get_count_slow(grid, offsets, &instructions), 39);

        let instructions = parse(INPUT2, Some((-50..=50,-50..=50,-50..=50)));
        let grid = Array3::from_elem([101,101,101], false);
        let offsets = [50,50,50];
        assert_eq!(get_count_slow(grid, offsets, &instructions), 590784);
    }

    #[test]
    fn test2() {
        let instructions = parse(INPUT, Some((-50..=50,-50..=50,-50..=50)));
        assert_eq!(get_count(&instructions), 39);

        let instructions = parse(INPUT2, Some((-50..=50,-50..=50,-50..=50)));
        assert_eq!(get_count(&instructions), 590784);
    }
}