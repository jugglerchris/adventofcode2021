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
enum Location<const ROOMSIZE: usize> {
    Corridor(usize),
    RoomA(usize),
    RoomB(usize),
    RoomC(usize),
    RoomD(usize),
}

impl<const ROOMSIZE: usize> Location<{ROOMSIZE}> {
    fn from_usize(mut u: usize) -> Self {
        if u <= 10 {
            return Location::Corridor(u);
        }
        u -= 11;
        if u < ROOMSIZE {
            return Location::RoomA(u);
        }
        u -= ROOMSIZE;
        if u < ROOMSIZE {
            return Location::RoomB(u);
        }
        u -= ROOMSIZE;
        if u < ROOMSIZE {
            return Location::RoomC(u);
        }
        u -= ROOMSIZE;
        if u < ROOMSIZE {
            return Location::RoomD(u);
        }
        panic!()
    }
    fn to_usize(&self) -> usize {
        use Location::*;
        match *self {
            Corridor(n) => n as usize,
            RoomA(n) => 11 + n as usize,
            RoomB(n) => 11 + (ROOMSIZE * 1) + n as usize,
            RoomC(n) => 11 + (ROOMSIZE * 2) + n as usize,
            RoomD(n) => 11 + (ROOMSIZE * 3) + n as usize,
        }
    }

    pub(crate) fn is_dest(&self, amphipod: Amphipod) -> bool {
        use Amphipod::*;
        use Location::*;
        match (amphipod, *self) {
            (Amber, RoomA(_)) => true,
            (Bronze, RoomB(_)) => true,
            (Copper, RoomC(_)) => true,
            (Desert, RoomD(_)) => true,
            _ => false,
        }
    }

    pub(crate) fn is_dest_corridor(&self) -> bool {
        match *self {
            Location::Corridor(2) |
            Location::Corridor(4) |
            Location::Corridor(6) |
            Location::Corridor(8) => false,
            Location::Corridor(n) => true,
            _ => false,
        }
    }


    fn coords(&self) -> (isize, isize) {
        match *self {
            Location::RoomA(p) => (2, p as isize +1),
            Location::RoomB(p) => (4, p as isize +1),
            Location::RoomC(p) => (6, p as isize +1),
            Location::RoomD(p) => (8, p as isize +1),
            Location::Corridor(p) => (p as isize , 0),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Burrow<const ROOMSIZE: usize> {
    spots: [Option<Amphipod>; /* 11 + 4*ROOMSIZE*/ 27],
}

impl<const ROOMSIZE: usize> Burrow<ROOMSIZE> {
    const fn len(&self) -> usize {
        11 + 4*ROOMSIZE
    }
}

type Data = Burrow<2>;
fn parse_input(input: &str) -> Data {
    let mut burrow = Burrow::<2> {
        spots: [None; 11+4*4],
    };
    let mut lines = input.lines();
    lines.next();
    lines.next();
    use Location::*;
    let top = lines.next().unwrap().as_bytes();
    burrow.spots[RoomA::<2>(0).to_usize()] = Some(Amphipod::from_byte(top[3]));
    burrow.spots[RoomB::<2>(0).to_usize()] = Some(Amphipod::from_byte(top[5]));
    burrow.spots[RoomC::<2>(0).to_usize()] = Some(Amphipod::from_byte(top[7]));
    burrow.spots[RoomD::<2>(0).to_usize()] = Some(Amphipod::from_byte(top[9]));
    let bot = lines.next().unwrap().as_bytes();
    burrow.spots[RoomA::<2>(1).to_usize()] = Some(Amphipod::from_byte(bot[3]));
    burrow.spots[RoomB::<2>(1).to_usize()] = Some(Amphipod::from_byte(bot[5]));
    burrow.spots[RoomC::<2>(1).to_usize()] = Some(Amphipod::from_byte(bot[7]));
    burrow.spots[RoomD::<2>(1).to_usize()] = Some(Amphipod::from_byte(bot[9]));
    burrow
}

#[derive(Clone, PartialEq, Eq)]
struct State<const ROOMSIZE: usize> {
    cost: usize,
    min_cost: usize,
    burrow: Burrow<ROOMSIZE>,
}

impl<const ROOMSIZE: usize> std::fmt::Debug for State<ROOMSIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Location::*;
        writeln!(f, "Cost: {}, min cost {} total {}", self.cost, self.min_cost, self.cost + self.min_cost)?;
        writeln!(f, "#############")?;
        write!(f, "#")?;
        let tochar = |loc: Location<ROOMSIZE>| {
            if let Some(amph) = self.burrow.spots[loc.to_usize()] {
                amph.as_char()
            } else {
                '.'
            }
        };
        for pos in 0..=10 {
            write!(f, "{}", tochar(Corridor(pos)))?;
        }
        writeln!(f, "#")?;
        writeln!(f, "###{}#{}#{}#{}###", tochar(RoomA(0)), tochar(RoomB(0)), tochar(RoomC(0)), tochar(RoomD(0)))?;
        for n in 1..ROOMSIZE {
            writeln!(f, "  #{}#{}#{}#{}#  ", tochar(RoomA(n)), tochar(RoomB(n)), tochar(RoomC(n)), tochar(RoomD(n)))?;
        }
        writeln!(f, "  #########  ")?;
        Ok(())
    }
}

impl<const ROOMSIZE: usize> Ord for State<ROOMSIZE> {
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

impl<const ROOMSIZE: usize> PartialOrd for State<ROOMSIZE> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const ROOMSIZE: usize> State<ROOMSIZE> {
    fn new(burrow: &Burrow<ROOMSIZE>) -> Self {
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
        use Location::*;
        for i in 0..ROOMSIZE {
            if self.burrow.spots[RoomA::<ROOMSIZE>(i).to_usize()] != Some(Amber) {
                return false;
            }
            if self.burrow.spots[RoomB::<ROOMSIZE>(i).to_usize()] != Some(Bronze) {
                return false;
            }
            if self.burrow.spots[RoomC::<ROOMSIZE>(i).to_usize()] != Some(Copper) {
                return false;
            }
            if self.burrow.spots[RoomD::<ROOMSIZE>(i).to_usize()] != Some(Desert) {
                return false;
            }
        }
        true
    }
    fn next_positions(&self, location: Location<ROOMSIZE>) -> Vec<Location<ROOMSIZE>> {
        use Location::*;
        match location {
            RoomA(0) => vec![RoomA(1), Corridor(2)],
            RoomA(n) if n==ROOMSIZE-1 => vec![RoomA(ROOMSIZE-2)],
            RoomA(n) => vec![RoomA(n-1), RoomA(n+1)],
            RoomB(0) => vec![RoomB(1), Corridor(4)],
            RoomB(n) if n==ROOMSIZE-1 => vec![RoomB(ROOMSIZE-2)],
            RoomB(n) => vec![RoomB(n-1), RoomB(n+1)],
            RoomC(0) => vec![RoomC(1), Corridor(6)],
            RoomC(n) if n==ROOMSIZE-1 => vec![RoomC(ROOMSIZE-2)],
            RoomC(n) => vec![RoomC(n-1), RoomC(n+1)],
            RoomD(0) => vec![RoomD(1), Corridor(8)],
            RoomD(n) if n==ROOMSIZE-1 => vec![RoomD(ROOMSIZE-2)],
            RoomD(n) => vec![RoomD(n-1), RoomD(n+1)],
            Corridor(0) => vec![Corridor(1)],
            Corridor(10) => vec![Corridor(9)],
            Corridor(n) => {
                let mut result = vec![Corridor(n-1), Corridor(n+1)];
                if n == 2 {
                    result.push(RoomA(0));
                }
                if n == 4 {
                    result.push(RoomB(0));
                }
                if n == 6 {
                    result.push(RoomC(0));
                }
                if n == 8 {
                    result.push(RoomD(0));
                }
                result
            }
        }
    }
    fn next_states(&self) -> Vec<Self> {
        let mut result = Vec::new();
        use Location::*;
        use Amphipod::*;
        for pos_u in 0..(self.burrow.len()) {
            let pos = Location::from_usize(pos_u);
            if let Some(amphipod) = self.burrow.spots[pos_u] {
                let (home0, home_n) = match amphipod {
                    Amber => (RoomA::<ROOMSIZE>(0).to_usize(), RoomA::<ROOMSIZE>(ROOMSIZE-1).to_usize()),
                    Bronze => (RoomB::<ROOMSIZE>(0).to_usize(), RoomB::<ROOMSIZE>(ROOMSIZE-1).to_usize()),
                    Copper => (RoomC::<ROOMSIZE>(0).to_usize(), RoomC::<ROOMSIZE>(ROOMSIZE-1).to_usize()),
                    Desert => (RoomD::<ROOMSIZE>(0).to_usize(), RoomD::<ROOMSIZE>(ROOMSIZE-1).to_usize()),
                };
                if pos_u >= home0 && pos_u <= home_n {
                    // In a home area
                    let mut blocked = false;
                    for u in pos_u+1..=home_n {
                        if self.burrow.spots[u] != Some(amphipod) {
                            blocked = true;
                            break;
                        }
                    }
                    if !blocked {
                        // It's home and doesn't need to leave.
                        continue;
                    }
                }
                let mut seen = HashSet::new();
                seen.insert(pos);
                let mut starting_points = vec![(pos, 0)];
                while let Some((p, cost)) = starting_points.pop() {
                    for newpos in self.next_positions(p) {
                        if !seen.contains(&newpos) && self.burrow.spots[newpos.to_usize()].is_none() {
                            seen.insert(newpos);
                            let newcost = cost + amphipod.move_cost();
                            starting_points.push((newpos, cost + amphipod.move_cost()));
                            if self.is_valid_move(amphipod, pos, newpos) {
                                result.push(self.make_move(pos, newpos, newcost));
                            }
                        }
                    }
                }
            }
        }
        result
    }

    fn is_valid_move(&self, amphipod: Amphipod, from: Location<ROOMSIZE>, to: Location<ROOMSIZE>) -> bool {
        use Location::*;
        use Amphipod::*;
        match from {
            RoomA(_) |
            RoomB(_) |
            RoomC(_) |
            RoomD(_) => to.is_dest_corridor(),
            Corridor(2) |
            Corridor(4) |
            Corridor(6) |
            Corridor(8) => false,
            Corridor(_n) => {
                if to.is_dest(amphipod) {
                    let (_home0, home_n) = match amphipod {
                        Amber => (RoomA::<ROOMSIZE>(0).to_usize(), RoomA::<ROOMSIZE>(ROOMSIZE-1).to_usize()),
                        Bronze => (RoomB::<ROOMSIZE>(0).to_usize(), RoomB::<ROOMSIZE>(ROOMSIZE-1).to_usize()),
                        Copper => (RoomC::<ROOMSIZE>(0).to_usize(), RoomC::<ROOMSIZE>(ROOMSIZE-1).to_usize()),
                        Desert => (RoomD::<ROOMSIZE>(0).to_usize(), RoomD::<ROOMSIZE>(ROOMSIZE-1).to_usize()),
                    };
//                    dbg!((amphipod, _home0, home_n, to, to.to_usize()));
                    for u in (to.to_usize()+1)..=home_n {
                        if self.burrow.spots[u] != Some(amphipod) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    fn make_move(&self, pos: Location<ROOMSIZE>, newpos: Location<ROOMSIZE>, newcost: usize) -> Self {
        let mut result = self.clone();
        assert!(result.burrow.spots[pos.to_usize()].is_some());
        assert!(result.burrow.spots[newpos.to_usize()].is_none());
        result.burrow.spots.swap(pos.to_usize(), newpos.to_usize());
        result.cost = self.cost + newcost;
        result.min_cost = result.min_cost();

//        dbg!(("make_move", self, pos, newpos, &result));

        result
    }

    fn min_cost(&self) -> usize {
        use Amphipod::*;
        use Location::*;
        let mut cost = 0;
        for loc_u in 0..self.burrow.len() {
            if let Some(amph) = self.burrow.spots[loc_u] {
                let loc = Location::from_usize(loc_u);
//                dbg!(loc);
                let is_home = match (amph, loc) {
                    (Amber, RoomA(_)) => true,
                    (Bronze, RoomB(_)) => true,
                    (Copper, RoomC(_)) => true,
                    (Desert, RoomD(_)) => true,
                    _ => false,
                };
                if is_home {
                    let (_, y) = loc.coords();
                    let mut is_really_home = true;
                    let bot_home: Location<ROOMSIZE> = match amph {
                        Amber => RoomA(ROOMSIZE-1),
                        Bronze => RoomB(ROOMSIZE-1),
                        Copper => RoomC(ROOMSIZE-1),
                        Desert => RoomD(ROOMSIZE-1),
                    };
                    for i in (y as usize)+1..=bot_home.to_usize() {
//                        dbg!((y, bot_home, bot_home.to_usize(), i));
                        if self.burrow.spots[i] != Some(amph) {
                            // Have to move out first
                            is_really_home = false;
                            break;
                        }
                    }
                    if !is_really_home {
                        cost += (y as usize + 1 + 1 + ROOMSIZE) * amph.move_cost();
                    }
                    continue;
                }
                let home = match amph {
                    Amber => RoomA(ROOMSIZE-1),
                    Bronze => RoomB(ROOMSIZE-1),
                    Copper => RoomC(ROOMSIZE-1),
                    Desert => RoomD(ROOMSIZE-1),
                };
//                dbg!(home);
                cost += amph.move_cost() * self.dist(loc, home);
            }
        }
        // TODO: account for slightly shorter distances actually required
        // since we tried to get everyone to the bottom space.
        cost
    }

    pub(crate) fn dist(&self, from: Location<ROOMSIZE>, to: Location<ROOMSIZE>) -> usize {
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
    let mut burrow4 = Burrow::<4> {
        spots: [None; 11+4*4],
    };
    {
        use Amphipod::*;
        use Location::*;
        for i in 0..=10 {
            burrow4.spots[i] = data.spots[i];
        }
        burrow4.spots[RoomA::<4>(0).to_usize()] = data.spots[RoomA::<2>(0).to_usize()];
        burrow4.spots[RoomA::<4>(3).to_usize()] = data.spots[RoomA::<2>(1).to_usize()];
        burrow4.spots[RoomB::<4>(0).to_usize()] = data.spots[RoomB::<2>(0).to_usize()];
        burrow4.spots[RoomB::<4>(3).to_usize()] = data.spots[RoomB::<2>(1).to_usize()];
        burrow4.spots[RoomC::<4>(0).to_usize()] = data.spots[RoomC::<2>(0).to_usize()];
        burrow4.spots[RoomC::<4>(3).to_usize()] = data.spots[RoomC::<2>(1).to_usize()];
        burrow4.spots[RoomD::<4>(0).to_usize()] = data.spots[RoomD::<2>(0).to_usize()];
        burrow4.spots[RoomD::<4>(3).to_usize()] = data.spots[RoomD::<2>(1).to_usize()];

        burrow4.spots[RoomA::<4>(1).to_usize()] = Some(Desert);
        burrow4.spots[RoomA::<4>(2).to_usize()] = Some(Desert);

        burrow4.spots[RoomB::<4>(1).to_usize()] = Some(Copper);
        burrow4.spots[RoomB::<4>(2).to_usize()] = Some(Bronze);

        burrow4.spots[RoomC::<4>(1).to_usize()] = Some(Bronze);
        burrow4.spots[RoomC::<4>(2).to_usize()] = Some(Amber);

        burrow4.spots[RoomD::<4>(1).to_usize()] = Some(Amber);
        burrow4.spots[RoomD::<4>(2).to_usize()] = Some(Copper);
    }

    let mut states = BinaryHeap::new();
    let mut seen = HashMap::new();
    states.push(State::new(&burrow4));
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

#[test]
fn test() {
    let tests = r#"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 12521);
    assert_eq!(part2(&data), 44169);
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
