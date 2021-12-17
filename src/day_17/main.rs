use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input");

    println!("{}",get_higest_y(input));

    println!("{}",get_velocities_count(input));


}

fn get_higest_y(input: &str) -> i32 {
    let re = Regex::new(r"(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)").unwrap();

    let cap = re.captures(input).unwrap();
    let xmin: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
    let xmax: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
    let ymin: i32 = cap.get(3).unwrap().as_str().parse().unwrap();
    let ymax: i32 = cap.get(4).unwrap().as_str().parse().unwrap();

    let x_target_range = xmin..=xmax;
    let y_target_range = ymin..=ymax;

    dbg!(&x_target_range);
    dbg!(&y_target_range);

    let mut y_max = 0;
    for y_vel_init in 0..1000 {
        let mut y_pos = 0;
        let mut y_vel = y_vel_init;
        let mut y_local_max = 0;
        while y_pos >= ymin {
            y_pos += y_vel;
            y_vel -= 1;
            if y_vel == 0 { y_local_max = y_pos }
            if y_target_range.contains(&y_pos) {
                y_max = y_max.max(y_local_max);
                continue
            }
        }
    }
    y_max
}

fn get_velocities_count(input: &str) -> usize {
    let re = Regex::new(r"(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)").unwrap();

    let cap = re.captures(input).unwrap();
    let xmin: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
    let xmax: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
    let ymin: i32 = cap.get(3).unwrap().as_str().parse().unwrap();
    let ymax: i32 = cap.get(4).unwrap().as_str().parse().unwrap();

    let x_target_range = xmin..=xmax;
    let y_target_range = ymin..=ymax;

    dbg!(&x_target_range);
    dbg!(&y_target_range);

    let mut y_hit_options = HashSet::new();
    for y_vel_init in -1000..1000 {
        let mut y_pos = 0;
        let mut y_vel = y_vel_init;
        let mut tick = 1;
        while y_pos >= ymin {
            y_pos += y_vel;
            y_vel -= 1;
            if y_target_range.contains(&y_pos) {
                y_hit_options.insert((tick,y_vel_init));
            }
            tick += 1;
        }
    }
    
    dbg!(y_hit_options.len());

    let mut velocities = HashSet::new();
    for (final_tick, y_vel_init) in y_hit_options.iter() {
        for x_vel_init in 0..=xmax {
            let mut x_pos = 0;
            let mut x_vel = x_vel_init;
            for _ in 1..=*final_tick {
                x_pos += x_vel;
                if x_vel > 0 { x_vel -= 1 } else if x_vel < 0 { x_vel += 1}
                if x_pos > xmax { break }
            }
            if x_target_range.contains(&x_pos) {
                velocities.insert((x_vel_init, *y_vel_init));
            }
        }
    }

    velocities.iter().count()
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test1() {
        assert_eq!(get_higest_y(INPUT), 45);
    }

    #[test]
    fn test2() {
        assert_eq!(get_velocities_count(INPUT), 112);
    }
}