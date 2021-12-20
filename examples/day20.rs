use adventofcode2021::get_input;

#[derive(Clone, Debug)]
struct Data {
    algo: Vec<bool>,
    image: Vec<Vec<bool>>,
    default: bool,
}

impl Data {
    pub fn step(&mut self) {
        let mut new_field = Vec::new();
        for y in -1isize..(self.image.len() as isize + 1) {
            let mut new_line = Vec::new();
            for x in -1..(self.image[0].len() as isize + 1) {
                let mut idx: usize = 0;
                for yy in (y-1)..=(y+1) {
                    for xx in (x-1)..=(x+1) {
                        idx <<= 1;
                        if self.get(xx, yy) {
                            idx |= 1;
                        }
                    }
                }
                new_line.push(self.algo[idx]);
            }
            new_field.push(new_line);
        }
        self.image = new_field;
        self.default = if self.default {
            self.algo[0o777]
        } else {
            self.algo[0]
        };
    }
    fn get(&self, x: isize, y: isize) -> bool {
        if x >=0 && y >= 0 {
            let x = x as usize;
            let y = y as usize;
            *self.image.get(y).and_then(|line| line.get(x)).unwrap_or(&self.default)
        } else {
            self.default
        }
    }
    fn count_set(&self) -> usize {
        let mut count = 0;
        for line in &self.image {
            count += line.iter().filter(|b| **b).count();
        }
        count
    }
    fn print(&self) {
        println!("Image:");
        for line in &self.image {
            for cell in line {
                if *cell {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

fn parse_input(input: &str) -> Data {
    let mut lines = input.lines()
                     .map(|s| {
                         s.chars()
                          .map(|c| c == '#')
                          .collect::<Vec<bool>>()
                     })
                    .collect::<Vec<_>>();

    assert_eq!(lines[0].len(), 512);
    assert_eq!(lines[1].len(), 0);
    let algo = lines.remove(0);
    lines.remove(0);
    Data {
        algo: algo,
        image: lines,
        default: false,
    }
}

fn part1(data: &Data) -> usize {
    let mut data = (*data).clone();
    /*#[cfg(test)]*/ data.print();
    data.step();
    /*#[cfg(test)]*/ data.print();
    data.step();
    /*#[cfg(test)]*/ data.print();
    dbg!(data.image.len(), data.image[0].len());
    data.count_set()
}
fn part2(data: &Data) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let tests = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 35);
    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(20)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
