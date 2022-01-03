use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Display;

fn main() {
    let input = include_str!("input");
    let (_, energy) = get_least_energy(&parse_part1(input));
    println!("{}", energy);

    let (_, energy) = get_least_energy(&parse_part2(input));
    println!("{}", energy);
}


fn get_least_energy<const R: usize>(initial_state: &State<R>) -> (Vec<(State<R>, u64)>, u64) {
    let mut queue = BinaryHeap::from([
        RatedState { encoded_state: initial_state.encode(), f_score: 0 }
    ]);
    let mut g_score_map = HashMap::from([
        (initial_state.encode(), 0)
    ]);
    let mut came_from = HashMap::from([
        (initial_state.encode(), None)
    ]);
    let encoded_goal = State::<R>::goal().encode();

    while let Some(RatedState { encoded_state, f_score }) = queue.pop() {
        if encoded_state == encoded_goal {
            let history: Vec<(State<R>, u64)> = std::iter::successors(Some(encoded_state), |e| {
                came_from[e]
            }).map(|e| (State::decode(e), g_score_map[&e]))
            .collect::<Vec<(State<R>, u64)>>().into_iter().rev().collect();
            return (history, f_score)
        }

        let cur_state = State::<R>::decode(encoded_state);
        let cur_g_score = g_score_map[&encoded_state];

        for (new_state, transition_cost) in cur_state.transitions() {
            let new_g_score = cur_g_score + transition_cost;
            let new_state_encoded = new_state.encode();
            if !g_score_map.contains_key(&new_state_encoded) || g_score_map[&new_state_encoded] > new_g_score {
                came_from.insert(new_state_encoded, Some(encoded_state));
                g_score_map.insert(new_state_encoded, new_g_score);
                let f_score = new_g_score + new_state.h_score();
                queue.push(RatedState {
                    encoded_state: new_state_encoded,
                    f_score,
                });
            }
        }
    }

    unreachable!()
}


#[derive(Clone, Copy, Debug)]
enum Amphipod {
    None = 0,
    A = 1,
    B = 2,
    C = 3,
    D = 4,
}

impl Amphipod {
    fn movement_cost(&self) -> u64 {
        match self {
            Amphipod::None => 0,
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }

    fn desired_room_idx(&self) -> usize {
        match self {
            Amphipod::None => unreachable!(),
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3,
        }
    }
}


#[derive(Clone, Debug)]
struct State<const R: usize> {
    hallway: [Amphipod; 11],
    rooms: [[Amphipod; R]; 4],
}

impl<const R: usize> State<R> {
    /*fn from(input: &str) -> Self {
        let amphipods: Vec<Amphipod> = input.chars()
            .filter_map(|c| match c {
                'A' => Some(Amphipod::A),
                'B' => Some(Amphipod::B),
                'C' => Some(Amphipod::C),
                'D' => Some(Amphipod::D),
                _ => None,
            }).collect();
        
        if R == 2 {
            Self {
                hallway: [Amphipod::None; 11],
                rooms: [
                    [amphipods[0], amphipods[4]],
                    [amphipods[1], amphipods[5]],
                    [amphipods[2], amphipods[6]],
                    [amphipods[3], amphipods[7]]
                ],
            }
        } else if R == 4 {
            Self {
                hallway: [Amphipod::None; 11],
                rooms: [
                    [amphipods[0], Amphipod::D, Amphipod::D, amphipods[4]],
                    [amphipods[1], Amphipod::C, Amphipod::B, amphipods[5]],
                    [amphipods[2], Amphipod::B, Amphipod::A, amphipods[6]],
                    [amphipods[3], Amphipod::A, Amphipod::C, amphipods[7]]
                ],
            }
        } else {
            unimplemented!()
        }
    }*/

    fn encode(&self) -> u64 {
        self.rooms.iter().flatten().rev()
            .chain(self.hallway.iter().rev())
            .map(|a| *a as u64 )
            .fold(0, |acc, v| acc * 5 + v)
    }

    fn decode(mut val: u64) -> Self {
        let mut arr = std::iter::from_fn(|| {
            let decoded = val % 5;
            val /= 5;
            match decoded {
                0 => Some(Amphipod::None),
                1 => Some(Amphipod::A),
                2 => Some(Amphipod::B),
                3 => Some(Amphipod::C),
                4 => Some(Amphipod::D),
                _ => panic!("Invalid decode value"),
            }
        });
        Self {
            hallway: [(); 11].map(|_| arr.next().unwrap()),
            rooms: [(); 4].map(|_| [(); R].map(|_| arr.next().unwrap())),
        }
    }

    fn transitions(&self) -> Vec<(Self, u64)> {
        let mut transitions = Vec::new();

        // Room -> Hallway
        // Cannot end in space over room
        for (room_idx, room) in self.rooms.iter().enumerate() {
            if !self.can_exit_room(room_idx) { continue }
            // Get first non-empty space in room
            let (room_depth, amp) = room.iter().enumerate().find_map(|(room_depth, amp)| match amp {
                Amphipod::None => None,
                _ => Some((room_depth, amp)),
            }).unwrap();
            let hallway_start_idx = room_idx * 2 + 2;
            for hallway_idx in (0..hallway_start_idx).rev() {
                if !matches!(self.hallway[hallway_idx], Amphipod::None) { break }
                if State::<R>::invalid_hallway_indexes().contains(&hallway_idx) { continue }
                let movement = room_depth + 1 + (hallway_start_idx - hallway_idx);
                let cost = movement as u64 * amp.movement_cost();
                let mut state_copy = self.clone();
                std::mem::swap(
                    &mut state_copy.rooms[room_idx][room_depth],
                    &mut state_copy.hallway[hallway_idx]);
                transitions.push((state_copy, cost));
            }
            for hallway_idx in (hallway_start_idx+1)..self.hallway.len() {
                if !matches!(self.hallway[hallway_idx], Amphipod::None) { break }
                if State::<R>::invalid_hallway_indexes().contains(&hallway_idx) { continue }
                let movement = room_depth + 1 + (hallway_idx - hallway_start_idx);
                let cost = movement as u64 * amp.movement_cost();
                let mut state_copy = self.clone();
                std::mem::swap(
                    &mut state_copy.rooms[room_idx][room_depth],
                    &mut state_copy.hallway[hallway_idx]);
                transitions.push((state_copy, cost));
            }
        }

        // Hallway -> Room
        // Must be the correct room and contain only None or correct creatures
        for (hallway_idx, amp) in self.hallway.iter().enumerate() {
            if matches!(amp, Amphipod::None) { continue }
            if !self.can_enter_room(amp.desired_room_idx()) { continue }
            let desired_hallway_idx = amp.desired_room_idx() * 2 + 2;
            let (hallway_range, hallway_dist) = if hallway_idx < desired_hallway_idx {
                ((hallway_idx+1)..desired_hallway_idx,
                desired_hallway_idx-hallway_idx)
            } else {
                ((desired_hallway_idx+1)..hallway_idx,
                hallway_idx-desired_hallway_idx)
            };
            if !hallway_range.map(|idx| self.hallway[idx]).all(|a| matches!(a,Amphipod::None)) {
                continue
            }
            // Get last empty space in room
            let room = &self.rooms[amp.desired_room_idx()];
            let room_depth = room.iter().enumerate().rev().find_map(|(room_depth, amp)| match amp {
                Amphipod::None => Some(room_depth),
                _ => None,
            }).unwrap();
            let movement = hallway_dist + room_depth + 1;
            let cost = movement as u64 * amp.movement_cost();
            let mut state_copy = self.clone();
            std::mem::swap(
                &mut state_copy.rooms[amp.desired_room_idx()][room_depth],
                &mut state_copy.hallway[hallway_idx]);
            transitions.push((state_copy, cost));
        }
        
        transitions
    }

    fn can_enter_room(&self, room_idx: usize) -> bool {
        self.rooms[room_idx].iter().all(|a| match a {
            Amphipod::None => true,
            _ => a.desired_room_idx() == room_idx,
        })
    }

    fn can_exit_room(&self, room_idx: usize) -> bool {
        !self.can_enter_room(room_idx)
    }

    fn h_score(&self) -> u64 {
        let mut score = 0;

        // Min cost to move all creatures out of incorrect rooms into space above correct room
        for (room_idx, room) in self.rooms.iter().enumerate() {
            if !self.can_exit_room(room_idx) { continue }
            let mut blocker = false;
            let cur_room_hallway_idx = room_idx * 2 + 2;
            for (room_depth, amp) in room.iter().enumerate().rev() {
                if matches!(amp, Amphipod::None) { break }
                if !blocker && amp.desired_room_idx() == room_idx { continue }
                else if !blocker { blocker = true }
                let desired_hallway_idx = (amp.desired_room_idx()) * 2 + 2;
                let hallway_movement = (if desired_hallway_idx > cur_room_hallway_idx {
                    desired_hallway_idx - cur_room_hallway_idx
                } else {
                    cur_room_hallway_idx - desired_hallway_idx
                }).max(2);
                score += ((room_depth + 1 + hallway_movement) as u64) * amp.movement_cost();
                if amp.desired_room_idx() == room_idx {
                    score += room_depth as u64 + 1;
                }
            }
        }

        // Cost for creatures in hallway to move to space above room
        for (hallway_idx, amp) in self.hallway.iter().enumerate() {
            if matches!(amp, Amphipod::None) { continue }
            let desired_hallway_idx = amp.desired_room_idx() * 2 + 2;
            let hallway_movement = if desired_hallway_idx > hallway_idx {
                desired_hallway_idx - hallway_idx
            } else {
                hallway_idx - desired_hallway_idx
            };
            score += (hallway_movement as u64) * amp.movement_cost();
        }

        // Cost to move from above proper room into spot
        for (room_idx, room) in self.rooms.iter().enumerate() {
            for (room_depth, amp) in room.iter().enumerate() {
                if matches!(amp, Amphipod::None) || amp.desired_room_idx() == room_idx { continue }
                score += (room_depth as u64 + 1) * amp.movement_cost();
            }
        }

        score
    }

    const fn goal() -> Self {
        Self {
            hallway: [Amphipod::None; 11],
            rooms: [
                [Amphipod::A; R],
                [Amphipod::B; R],
                [Amphipod::C; R],
                [Amphipod::D; R],
            ]
        }
    }

    const fn invalid_hallway_indexes() -> [usize; 4] {
        [2, 4, 6, 8]
    }
}

fn parse_part1(input: &str) -> State<2> {
    let amphipods: Vec<Amphipod> = input.chars()
        .filter_map(|c| match c {
            'A' => Some(Amphipod::A),
            'B' => Some(Amphipod::B),
            'C' => Some(Amphipod::C),
            'D' => Some(Amphipod::D),
            _ => None,
        }).collect();
    
    State {
        hallway: [Amphipod::None; 11],
        rooms: [
            [amphipods[0], amphipods[4]],
            [amphipods[1], amphipods[5]],
            [amphipods[2], amphipods[6]],
            [amphipods[3], amphipods[7]]
        ],
    }
}

fn parse_part2(input: &str) -> State<4> {
    let amphipods: Vec<Amphipod> = input.chars()
        .filter_map(|c| match c {
            'A' => Some(Amphipod::A),
            'B' => Some(Amphipod::B),
            'C' => Some(Amphipod::C),
            'D' => Some(Amphipod::D),
            _ => None,
        }).collect();
    
    State {
        hallway: [Amphipod::None; 11],
        rooms: [
            [amphipods[0], Amphipod::D, Amphipod::D, amphipods[4]],
            [amphipods[1], Amphipod::C, Amphipod::B, amphipods[5]],
            [amphipods[2], Amphipod::B, Amphipod::A, amphipods[6]],
            [amphipods[3], Amphipod::A, Amphipod::C, amphipods[7]]
        ],
    }
}

impl<const R: usize> Display for State<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let space_to_str = |space: Amphipod| -> &str {
            match space {
                Amphipod::None => ".",
                Amphipod::A => "A",
                Amphipod::B => "B",
                Amphipod::C => "C",
                Amphipod::D => "D",
            }
        };

        writeln!(f, "{}", "#".repeat(self.hallway.len() + 2))?;
        writeln!(f, "#{}#", self.hallway.map(space_to_str).join(""))?;
        writeln!(f, "###{}###", self.rooms.map(|r| space_to_str(r[0])).join("#"))?;
        for room_depth in 1..R {
            writeln!(f, "  #{}#  ", self.rooms.map(|r| space_to_str(r[room_depth])).join("#"))?;
        }
        write!(f, "  {}  ", "#".repeat(self.rooms.len() * 2 + 1))?;

        Ok(())
    }
}


#[derive(PartialEq, Eq, Debug)]
struct RatedState {
    encoded_state: u64,
    f_score: u64,
}

impl PartialOrd for RatedState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.f_score.cmp(&other.f_score).reverse())
    }
}

impl Ord for RatedState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f_score.cmp(&other.f_score).reverse()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "#############\n\
    #...........#\n\
    ###B#C#B#D###\n\
      #A#D#C#A#\n\
      #########";

    #[test]
    fn test1() {
        let (history, energy) = get_least_energy(&parse_part1(INPUT));

        let mut prev_e = 0;
        for (s, e) in history.iter() {
            println!("Energy use: {}",e-prev_e);
            prev_e = *e;
            println!("{}",s);
            println!();
        }


        assert_eq!(energy, 12521);
    }

    #[test]
    fn test2() {
        let (history, energy) = get_least_energy(&parse_part2(INPUT));

        let mut prev_e = 0;
        for (s, e) in history.iter() {
            println!("Energy use: {}",e-prev_e);
            prev_e = *e;
            println!("{}",s);
            println!();
        }


        assert_eq!(energy, 44169);
    }
}