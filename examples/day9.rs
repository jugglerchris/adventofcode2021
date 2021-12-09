use adventofcode2021::get_input;

fn part1(data: &Field) -> usize {
    let mut sum = 0;
    for (x, y) in data.low_points() {
        sum += data.get(x, y) + 1;
    }
    sum
}
fn part2(data: &Field) -> usize {
    unimplemented!()
}

struct Field {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Field {
    pub fn from(input: &str) -> Self {
        let lines = input.split_whitespace().collect::<Vec<_>>();
        let height = lines.len();
        let width = lines[0].len();
        let mut data = Vec::new();
        for b in input.as_bytes() {
            match b {
                b'0'..=b'9' => {
                    data.push(b - b'0');
                }
                _ => (),
            }
        }
        Field { data, width, height }
    }

    pub fn try_get(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.data[x + y*self.width] as usize)
    }
    pub fn get(&self, x: usize, y: usize) -> usize {
        self.try_get(x, y).unwrap()
    }

    pub fn low_points(&self) -> impl Iterator<Item=(usize, usize)> {
        let mut points = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let v = self.get(x, y);
                if y > 0 && self.get(x, y-1) <= v {
                    continue;
                }
                if y+1 < self.height && self.get(x, y+1) <= v {
                    continue;
                }
                if x > 0 && self.get(x-1, y) <= v {
                    continue;
                }
                if x+1 < self.width && self.get(x+1, y) <= v {
                    continue;
                }
                points.push((x, y));
            }
        }
        points.into_iter()
    }
}

#[test]
fn test() {
    let tests = r#"2199943210
3987894921
9856789892
8767896789
9899965678
"#;
    let data = Field::from(tests);

    assert_eq!(part1(&data), 15);
//    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(9)?;

    let data = Field::from(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
