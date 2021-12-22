use adventofcode2021::{get_input,parse_lines,regex_parser};
use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct Step {
    on: bool,
    x0: isize,
    x1: isize,
    y0: isize,
    y1: isize,
    z0: isize,
    z1: isize,
}

regex_parser!(parse_step: Step {
    POS = r#"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$"# => 
        |onoff: String,
         x0: isize, x1: isize,
         y0: isize, y1: isize,
         z0: isize, z1: isize | {
             let on = match &onoff[..] {
                 "on" => true,
                 "off" => false,
                 _ => panic!(),
             };
             Step { on, x0, x1, y0, y1, z0, z1 }
         }
});

type Data = Vec<Step>;

fn parse_input(input: &str) -> Data {
    parse_lines(input)
}

fn part1(data: &Data) -> usize {
    let mut cubes = HashMap::new();
    for step in data {
        let x0 = step.x0.max(-50);
        let x1 = step.x1.min(50);
        let y0 = step.y0.max(-50);
        let y1 = step.y1.min(50);
        let z0 = step.z0.max(-50);
        let z1 = step.z1.min(50);
        for z in z0..=z1 {
            for y in y0..=y1 {
                for x in x0..=x1 {
                    cubes.insert((x, y, z), step.on);
                }
            }
        }
    }
    cubes.into_values().filter(|b| *b).count()
}
fn part2(data: &Data) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let tests = r#"on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 590784);
    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(22)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
