use adventofcode2021::{get_input,parse_lines, regex_parser};
use std::collections::HashMap;

type Coord = isize;

#[derive(Clone, Debug)]
pub struct Line {
    x1: Coord,
    y1: Coord,
    x2: Coord,
    y2: Coord,
}

impl Line {
    pub fn is_horiz(&self) -> bool {
        self.y1 == self.y2
    }
    pub fn is_vert(&self) -> bool {
        self.x1 == self.x2
    }
    pub fn is_orthogonal(&self) -> bool {
        self.is_horiz() || self.is_vert()
    }
}

pub struct Field {
    field: HashMap<(Coord, Coord), usize>,
}

impl Field {
    pub fn new() -> Field {
        Field {
            field: HashMap::new()
        }
    }
    pub fn insert(&mut self, x: Coord, y: Coord) {
        *self.field.entry((x, y))
            .or_insert(0) += 1;
    }
    pub fn iter(&self) -> impl Iterator<Item=(&(Coord, Coord), &usize)> {
        self.field.iter()
    }
    pub fn print(&self) {
        let mut max_x = 0;
        let mut max_y = 0;
        for &(x, y) in self.field.keys() {
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
        for y in 0..=max_y {
            for x in 0..=max_x {
                match self.field.get(&(x, y)) {
                    None => {
                        print!(".");
                    }
                    Some(&v) => {
                        if v <= 9 {
                            print!("{}", v);
                        } else {
                            print!("#");
                        }
                    }
                }
            }
            println!("");
        }
    }
}

regex_parser!(parse_line: Line {
    LINE = r#"^(\d+),(\d+) -> (\d+),(\d+)$"# =>
        |x1: Coord, y1: Coord, x2: Coord, y2: Coord| Line { x1, y1, x2, y2 }
});

pub fn part1(lines: &[Line]) -> usize {
    let mut field = Field::new();
    {
        for line in lines {
            if line.is_horiz() {
                #[cfg(test)]
                println!("Horiz: {:?}", line);
                let x1 = line.x1.min(line.x2);
                let x2 = line.x1.max(line.x2);
                for x in x1..=x2 {
                    field.insert(x, line.y1);
                }
            } else if line.is_vert() {
                #[cfg(test)]
                println!("Vert: {:?}", line);
                let y1 = line.y1.min(line.y2);
                let y2 = line.y1.max(line.y2);
                for y in y1..=y2 {
                    field.insert(line.x1, y);
                }
            } else {
                #[cfg(test)]
                println!("Other: {:?}", line);
            }
        }
    }
    #[cfg(test)]
    field.print();
    let mut overlaps = 0;
    for (_, &n) in field.iter() {
        if n > 1 {
            overlaps += 1;
        }
    }
    return overlaps;
}

#[test]
fn test_part1() {
    let lines = parse_lines(r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#);
    assert_eq!(part1(&lines), 5);
}

fn main() -> std::io::Result<()>{
    let input = get_input(5)?;

    let lines: Vec<Line> = parse_lines(&input);

    // Part 1
    println!("{}", part1(&lines));

    // Part 2
    /*
    println!("{}", increments.iter()
                             .map(|mass| {
                                 let mut mass = *mass;
                                 let mut fuel = 0;
                                 while mass > 5 {
                                     let f = mass/3 - 2;
                                     fuel += f;
                                     mass = f;
                                 }
                                 fuel
                              })
                             .sum::<u32>());
    */

    Ok(())
}
