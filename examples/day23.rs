use adventofcode2021::get_input;
use std::collections::{BinaryHeap, HashSet, HashMap};

#[derive(Debug,Copy,Clone,Eq, PartialEq, Ord,PartialOrd, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn move_cost(&self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }
    fn from_byte(byte: u8) -> Self {
        use Amphipod::*;
        match byte {
            b'A' => Amber,
            b'B' => Bronze,
            b'C' => Copper,
            b'D' => Desert,
            _ => panic!(),
        }
    }
    fn as_char(&self) -> char {
        match *self {
            Amphipod::Amber => 'A',
            Amphipod::Bronze => 'B',
            Amphipod::Copper => 'C',
            Amphipod::Desert => 'D',
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[repr(u8)]
enum Location {
    RoomA1 = 0,
    RoomA2 = 1,
    RoomB1 = 2,
    RoomB2 = 3,
    RoomC1 = 4,
    RoomC2 = 5,
    RoomD1 = 6,
    RoomD2 = 7,
    CorLEnd = 8,
    CorL = 9,
    CorA = 10,
    CorAB = 11,
    CorB = 12,
    CorBC = 13,
    CorC = 14,
    CorCD = 15,
    CorD = 16,
    CorR = 17,
    CorREnd = 18,
}

impl Location {
    fn from_usize(u: usize) -> Self {
        Location::from_u8(u as u8)
    }

    fn from_u8(u: u8) -> Self {
        assert!(u <= (Location::CorREnd as u8));
        unsafe { std::mem::transmute(u) }
    }

    pub(crate) fn is_dest(&self, amphipod: Amphipod) -> bool {
        use Amphipod::*;
        use Location::*;
        match amphipod {
            Amber => *self == RoomA1 || *self == RoomA2,
            Bronze => *self == RoomB1 || *self == RoomB2,
            Copper => *self == RoomC1 || *self == RoomC2,
            Desert => *self == RoomD1 || *self == RoomD2,
        }
    }

    pub(crate) fn is_room(&self) -> bool {
        use Location::*;
        match *self {
            RoomA1 | RoomA2 |
            RoomB1 | RoomB2 |
            RoomC1 | RoomC2 |
            RoomD1 | RoomD2 => true,
            _ => false
        }
    }
    pub(crate) fn is_corridor(&self) -> bool {
        !self.is_room()
    }
    pub(crate) fn is_dest_corridor(&self) -> bool {
        match *self {
            Location::CorLEnd |
            Location::CorL |
            Location::CorAB |
            Location::CorBC |
            Location::CorCD |
            Location::CorR |
            Location::CorREnd => true,
            _ => false,
        }
    }


    fn coords(&self) -> (isize, isize) {
        match *self {
            Location::RoomA1 => (2, 1),
            Location::RoomA2 => (2, 2),
            Location::RoomB1 => (4, 1),
            Location::RoomB2 => (4, 2),
            Location::RoomC1 => (6, 1),
            Location::RoomC2 => (6, 2),
            Location::RoomD1 => (8, 1),
            Location::RoomD2 => (8, 2),
            Location::CorLEnd => (0, 0),
            Location::CorL => (1, 0),
            Location::CorA => (2, 0),
            Location::CorAB => (3, 0),
            Location::CorB => (4, 0),
            Location::CorBC => (5, 0),
            Location::CorC => (6, 0),
            Location::CorCD => (7, 0),
            Location::CorD => (8, 0),
            Location::CorR => (9, 0),
            Location::CorREnd => (10, 0),
        }
    }
}

#[test]
fn test_location() {
    assert_eq!(Location::RoomA1 as u8, 0);
    assert_eq!(Location::CorREnd as u8, 18);
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Burrow {
    spots: [Option<Amphipod>; Location::CorREnd as usize + 1],
}

type Data = Burrow;
fn parse_input(input: &str) -> Data {
    let mut burrow = Burrow {
        spots: [None; Location::CorREnd as usize + 1],
    };
    let mut lines = input.lines();
    lines.next();
    lines.next();
    use Location::*;
    let top = lines.next().unwrap().as_bytes();
    burrow.spots[RoomA1 as u8 as usize] = Some(Amphipod::from_byte(top[3]));
    burrow.spots[RoomB1 as u8 as usize] = Some(Amphipod::from_byte(top[5]));
    burrow.spots[RoomC1 as u8 as usize] = Some(Amphipod::from_byte(top[7]));
    burrow.spots[RoomD1 as u8 as usize] = Some(Amphipod::from_byte(top[9]));
    let bot = lines.next().unwrap().as_bytes();
    burrow.spots[RoomA2 as u8 as usize] = Some(Amphipod::from_byte(bot[3]));
    burrow.spots[RoomB2 as u8 as usize] = Some(Amphipod::from_byte(bot[5]));
    burrow.spots[RoomC2 as u8 as usize] = Some(Amphipod::from_byte(bot[7]));
    burrow.spots[RoomD2 as u8 as usize] = Some(Amphipod::from_byte(bot[9]));
    burrow
}

#[derive(Clone, PartialEq, Eq)]
struct State {
    cost: usize,
    min_cost: usize,
    burrow: Burrow,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Cost: {}, min cost {} total {}", self.cost, self.min_cost, self.cost + self.min_cost)?;
        writeln!(f, "#############")?;
        use Location::*;
        write!(f, "#")?;
        let tochar = |loc: Location| {
            if let Some(amph) = self.burrow.spots[loc as u8 as usize] {
                amph.as_char()
            } else {
                '.'
            }
        };
        for pos in &[CorLEnd, CorL, CorA, CorAB, CorB, CorBC, CorC, CorCD, CorD, CorR, CorREnd] {
            write!(f, "{}", tochar(*pos))?;
        }
        writeln!(f, "#")?;
        writeln!(f, "###{}#{}#{}#{}###", tochar(RoomA1), tochar(RoomB1), tochar(RoomC1), tochar(RoomD1))?;
        writeln!(f, "  #{}#{}#{}#{}#  ", tochar(RoomA2), tochar(RoomB2), tochar(RoomC2), tochar(RoomD2))?;
        writeln!(f, "  #########  ")?;
        Ok(())
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Backwards cost sort
        let min_total_cost = self.cost + self.min_cost;
        let other_min_total_cost = other.cost + other.min_cost;
        other_min_total_cost.cmp(&min_total_cost)
            .then_with(|| other.cost.cmp(&self.cost))
            .then_with(||
                    self.burrow.cmp(&other.burrow))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn new(burrow: &Burrow) -> State {
        let mut result = State {
            cost: 0,
            burrow: burrow.clone(),
            min_cost: 0,
        };
        result.min_cost = result.min_cost();
        result
    }

    fn is_finished(&self) -> bool {
        use Amphipod::*;
        &self.burrow.spots[(Location::RoomA1 as usize)..=(Location::RoomD2 as usize)] == &[
            Some(Amber), Some(Amber),
            Some(Bronze), Some(Bronze),
            Some(Copper), Some(Copper),
            Some(Desert), Some(Desert),
        ][..]
    }
    fn next_positions(&self, location: Location) -> &'static [Location] {
        use Location::*;
        match location {
            RoomA1 => &[RoomA2, CorA],
            RoomA2 => &[RoomA1],
            RoomB1 => &[RoomB2, CorB],
            RoomB2 => &[RoomB1],
            RoomC1 => &[RoomC2, CorC],
            RoomC2 => &[RoomC1],
            RoomD1 => &[RoomD2, CorD],
            RoomD2 => &[RoomD1],
            CorLEnd => &[CorL],
            CorL => &[CorLEnd, CorA],
            CorAB => &[CorA, CorB],
            CorBC => &[CorB, CorC],
            CorCD => &[CorC, CorD],
            CorR => &[CorD, CorREnd],
            CorREnd => &[CorR],
            CorA => &[CorL, RoomA1, CorAB],
            CorB => &[CorAB, RoomB1, CorBC],
            CorC => &[CorBC, RoomC1, CorCD],
            CorD => &[CorCD, RoomD1, CorR],
        }
    }
    fn next_states(&self) -> Vec<State> {
        let mut result = Vec::new();
        use Location::*;
        use Amphipod::*;
        for pos_u in 0..(self.burrow.spots.len()) {
            let pos = Location::from_usize(pos_u);
            if let Some(amphipod) = self.burrow.spots[pos_u] {
                let (home1, home2) = match amphipod {
                    Amber => (RoomA1, RoomA2),
                    Bronze => (RoomB1, RoomB2),
                    Copper => (RoomC1, RoomC2),
                    Desert => (RoomD1, RoomD2),
                };
                if pos == home2 || (pos == home1 && self.burrow.spots[home2 as u8 as usize] == Some(amphipod)) {
                    // This one is already home
                    continue;
                }
                let mut seen = HashSet::new();
                seen.insert(pos);
                let mut starting_points = vec![(pos, 0)];
                while let Some((p, cost)) = starting_points.pop() {
                    for newpos in self.next_positions(p) {
                        if !seen.contains(&newpos) && self.burrow.spots[*newpos as usize].is_none() {
                            seen.insert(*newpos);
                            let newcost = cost + amphipod.move_cost();
                            starting_points.push((*newpos, cost + amphipod.move_cost()));
                            if self.is_valid_move(amphipod, pos, *newpos) {
                                result.push(self.make_move(pos, *newpos, newcost));
                            }
                        }
                    }
                }
            }
        }
        result
    }

    fn is_valid_move(&self, amphipod: Amphipod, from: Location, to: Location) -> bool {
        use Location::*;
        match from {
            RoomA1 | RoomA2 |
            RoomB1 | RoomB2 |
            RoomC1 | RoomC2 |
            RoomD1 | RoomD2 => to.is_dest_corridor(),
            CorLEnd |
            CorL |
            CorAB |
            CorBC |
            CorCD |
            CorR |
            CorREnd => {
                if to.is_dest(amphipod) {
                    match amphipod {
                        Amphipod::Amber => to == RoomA2 || self.burrow.spots[RoomA2 as u8 as usize].is_some(),
                        Amphipod::Bronze => to == RoomB2 || self.burrow.spots[RoomB2 as u8 as usize].is_some(),
                        Amphipod::Copper => to == RoomC2 || self.burrow.spots[RoomC2 as u8 as usize].is_some(),
                        Amphipod::Desert => to == RoomD2 || self.burrow.spots[RoomD2 as u8 as usize].is_some(),
                    }
                } else {
                    false
                }
            }
            CorA |
            CorB |
            CorC |
            CorD => false,
        }
    }

    fn make_move(&self, pos: Location, newpos: Location, newcost: usize) -> State {
        let mut result = self.clone();
        assert!(result.burrow.spots[pos as usize].is_some());
        assert!(result.burrow.spots[newpos as usize].is_none());
        result.burrow.spots.swap(pos as usize, newpos as usize);
        result.cost = self.cost + newcost;
        result.min_cost = result.min_cost();

        result
    }

    fn min_cost(&self) -> usize {
        use Amphipod::*;
        use Location::*;
        let mut cost = 0;
        for loc_u in 0..=(CorREnd as u8 as usize) {
            if let Some(amph) = self.burrow.spots[loc_u] {
                let loc = Location::from_usize(loc_u);
//                dbg!(loc);
                let is_home = match (amph, loc) {
                    (Amber, RoomA1|RoomA2) => true,
                    (Bronze, RoomB1|RoomB2) => true,
                    (Copper, RoomC1|RoomC2) => true,
                    (Desert, RoomD1|RoomD2) => true,
                    _ => false,
                };
                if is_home {
                    let (_, y) = loc.coords();
                    if y == 1 {
                        let bot_home = match amph {
                            Amber => RoomA2,
                            Bronze => RoomB2,
                            Copper => RoomC2,
                            Desert => RoomD2,
                            _ => panic!(),
                        };
                        if self.burrow.spots[bot_home as usize].unwrap() == amph {
                            // No cost - we're home
                        } else {
                            // Cost of moving out and back in again.
                            cost += (2 + 3) * amph.move_cost();
                        }
                    }
                    continue;
                }
                let home = match amph {
                    Amber => RoomA2,
                    Bronze => RoomB2,
                    Copper => RoomC2,
                    Desert => RoomD2,
                };
//                dbg!(home);
                cost += amph.move_cost() * self.dist(loc, home);
            }
        }
        // Subtract one move per type as we tried to get them all to the bottom spot.
//        dbg!(cost)
        // We've over counted by trying to move every amphipod to the bottom
        // of their room.  Subtract one movement for each, unless they were already
        // both home.
        if self.burrow.spots[RoomA1 as usize] != Some(Amber) ||
           self.burrow.spots[RoomA2 as usize] != Some(Amber) {
            cost -= Amber.move_cost();
        }
        if self.burrow.spots[RoomB1 as usize] != Some(Bronze) ||
           self.burrow.spots[RoomB2 as usize] != Some(Bronze) {
            cost -= Bronze.move_cost();
        }
        if self.burrow.spots[RoomC1 as usize] != Some(Copper) ||
           self.burrow.spots[RoomC2 as usize] != Some(Copper) {
            cost -= Copper.move_cost();
        }
        if self.burrow.spots[RoomD1 as usize] != Some(Desert) ||
           self.burrow.spots[RoomD2 as usize] != Some(Desert) {
            cost -= Desert.move_cost();
        }
        cost
    }

    pub(crate) fn dist(&self, from: Location, to: Location) -> usize {
//        dbg!((from, to));
        let (x0, y0) = from.coords();
        let (x1, y1) = to.coords();
//        dbg!((x0, y0));
//        dbg!((x1, y1));
        ((x1 - x0).abs() + y0 + y1) as usize
    }
}

fn part1(data: &Data) -> usize {
    let mut states = BinaryHeap::new();
    let mut seen = HashMap::new();
    states.push(State::new(data));
    loop {
        let state = states.pop().unwrap();
        #[cfg(test)]
        dbg!(&state);
        if state.is_finished() {
            return state.cost;
        }
        for newstate in state.next_states() {
            if newstate.cost < *seen.get(&newstate.burrow).unwrap_or(&usize::MAX) {
                seen.insert(newstate.burrow.clone(), newstate.cost);

                states.push(newstate);
            }
        }
    }
}
fn part2(data: &Data) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let tests = r#"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 12521);
    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(23)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
