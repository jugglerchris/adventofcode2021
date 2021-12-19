use adventofcode2021::{get_input,parse_lines,regex_parser};
use std::collections::{HashSet,HashMap};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    pub fn manhattan(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}
impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

regex_parser!(parse_point: Point {
   PT = r#"^(-?\d+),(-?\d+),(-?\d+)$"# => |x: isize, y:isize, z:isize| Point{ x, y, z}
});

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Point>,
    beacons_set: HashSet<Point>,
    // Set of distances between points, as sorted absolute values,
    // mapped to indices into beacons.
    distances: HashMap<(isize, isize, isize), (usize, usize)>,
}

fn map_point_dir(pt: Point, dir: u8) -> Point {
    match dir {
        0 => Point { x: pt.x, y: pt.y, z: pt.z },
        1 => Point { x: pt.x, y: pt.z, z: -pt.y },
        2 => Point { x: pt.x, y: -pt.y, z: -pt.z },
        3 => Point { x: pt.x, y: -pt.z, z: pt.y },

        4 => Point { x: -pt.x, y: -pt.y, z: pt.z },
        5 => Point { x: -pt.x, y: -pt.z, z: -pt.y },
        6 => Point { x: -pt.x, y: pt.y, z: -pt.z },
        7 => Point { x: -pt.x, y: pt.z, z: pt.y },

        8  => Point { x: pt.y, y: -pt.x, z: pt.z },
        9  => Point { x: pt.y, y: pt.z, z: pt.x },
        10 => Point { x: pt.y, y: pt.x, z: -pt.z },
        11 => Point { x: pt.y, y: -pt.z, z: -pt.x },

        12 => Point { x: -pt.y, y: pt.x, z: pt.z },
        13 => Point { x: -pt.y, y: -pt.z, z: pt.x },
        14 => Point { x: -pt.y, y: -pt.x, z: -pt.z },
        15 => Point { x: -pt.y, y: pt.z, z: -pt.x },

        16 => Point { x: pt.z, y: pt.y, z: -pt.x },
        17 => Point { x: pt.z, y: -pt.x, z: -pt.y },
        18 => Point { x: pt.z, y: -pt.y, z: pt.x },
        19 => Point { x: pt.z, y: pt.x, z: pt.y },

        20 => Point { x: -pt.z, y: -pt.y, z: -pt.x },
        21 => Point { x: -pt.z, y: pt.x, z: -pt.y },
        22 => Point { x: -pt.z, y: pt.y, z: pt.x },
        23 => Point { x: -pt.z, y: -pt.x, z: pt.y },

        _ => panic!()
    }
}

fn get_map(from: (Point, Point), to: (Point, Point)) -> Option<(u8, Point)>
{
    #[cfg(test)]
    dbg!(from);
    #[cfg(test)]
    dbg!(to);
    let d1 = from.1 - from.0;
    #[cfg(test)]
    dbg!(d1);
    let d2 = to.1 - to.0;
    #[cfg(test)]
    dbg!(d2);
    let mut dir = 0xff;
    for d in 0..24 {
        if map_point_dir(d2, d) == d1 {
            dir = d;
            break;
        }
    }
    #[cfg(test)]
    dbg!(dir);
    if dir >= 24 {
        return None;
    }
    assert!(dir < 24);
    let offset = from.0 - map_point_dir(to.0, dir);
    #[cfg(test)]
    dbg!(offset);
    assert_eq!(map_point_dir(to.1, dir) + offset, from.1);
    Some((dir, offset))
}

impl Scanner {
    pub fn new(beacons: Vec<Point>) -> Self {
        let mut distances = HashMap::new();
        for (i, a) in beacons.iter().enumerate() {
            for (j, b) in beacons.iter().enumerate() {
                if j > i {
                    let dx = (a.x - b.x).abs();
                    let dy = (a.y - b.y).abs();
                    let dz = (a.z - b.z).abs();
                    let mut v = vec![dx, dy, dz];
                    v.sort();
                    distances.insert((v[0], v[1], v[2]), (i, j));
                }
            }
        }
        let beacons_set = beacons.iter().cloned().collect();
        Scanner { beacons, beacons_set, distances }
    }

    pub fn num_distance_overlaps(&self, other: &Scanner) -> usize {
        let mut count = 0;
        for d in self.distances.keys() {
            if other.distances.contains_key(d) {
                count += 1;
            }
        }
        count
    }

    // returns other scanner's position
    pub fn merge(&mut self, other: Scanner) -> Point {
        for (d, my_idx) in &self.distances {
            if let Some(other_idx) = other.distances.get(d) {
                // Possibly matching pairs of points
                let do1 = get_map(
                    (self.beacons[my_idx.0],
                     self.beacons[my_idx.1]),
                    (other.beacons[other_idx.0],
                     other.beacons[other_idx.1]));
                let do2 = get_map(
                    (self.beacons[my_idx.0],
                     self.beacons[my_idx.1]),
                    (other.beacons[other_idx.1],
                     other.beacons[other_idx.0]));
                assert!(do1.is_some() || do2.is_some());
                assert!(do1.is_none() || do2.is_none());
                let (dir, offset) = do1.or(do2).unwrap();
                let orig_num_beacons = self.beacons.len();
                for &pt in &other.beacons {
                    let new_pt = map_point_dir(pt, dir) + offset;
                    self.beacons_set.insert(new_pt);
                    self.beacons.push(new_pt);
                }
                for (&k,&(i, j)) in other.distances.iter() {
                    self.distances.insert(k, (i+orig_num_beacons, j+orig_num_beacons));
                }
                return map_point_dir(Point { x:0, y:0, z:0 }, dir) + offset;
            }
        }
        panic!()
    }
}

type Data = Vec<Scanner>;
fn parse_input(input: &str) -> Data {
    let scanner_strings = input.split("\n\n");
    let mut result = Vec::new();
    for ss in scanner_strings {
        let coord_strings = ss.split_once('\n').unwrap().1;
        let beacons: Vec<Point> = parse_lines(coord_strings);
        result.push(Scanner::new(beacons));
    }
    result
}

fn part1(data: &Data) -> usize {
    #[cfg(test)]
    println!("{} scanners", data.len());

    let mut scanners = data.iter()
                           .cloned()
                           .enumerate()
                           .collect::<Vec<_>>();
    let (_, mut map) = scanners.remove(0);
    while !scanners.is_empty() {
        let mut other_idx = None;
        for (idx, (_i, other)) in scanners.iter().enumerate() {
            if map.num_distance_overlaps(other) >= 66 {
                other_idx = Some(idx);
                break;
            }
        };
        let (_i, other) = scanners.remove(other_idx.unwrap());
        map.merge(other);
    }
    map.beacons_set.len()
}
fn part2(data: &Data) -> isize {
    #[cfg(test)]
    println!("{} scanners", data.len());

    let mut scanners = data.iter()
                           .cloned()
                           .enumerate()
                           .collect::<Vec<_>>();
    let (_, mut map) = scanners.remove(0);
    let mut locations = vec![Point { x: 0, y: 0, z: 0 }];
    while !scanners.is_empty() {
        let mut other_idx = None;
        for (idx, (_i, other)) in scanners.iter().enumerate() {
            if map.num_distance_overlaps(other) >= 66 {
                other_idx = Some(idx);
                break;
            }
        };
        let (_i, other) = scanners.remove(other_idx.unwrap());
        locations.push(map.merge(other));
    }
    locations.iter()
             .map(|&pt| locations.iter()
                                .map(|&pt2| (pt2-pt).manhattan())
                                .max()
                                .unwrap())
             .max()
             .unwrap()
}

#[test]
fn test() {
    let tests = r#"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 79);
    assert_eq!(part2(&data), 3621);
}

fn main() -> std::io::Result<()>{
    let input = get_input(19)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
