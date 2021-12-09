use adventofcode2021::get_input;
use std::collections::HashMap;

fn part1(data: &Field) -> usize {
    let mut sum = 0;
    for (x, y) in data.low_points() {
        sum += data.get(x, y) + 1;
    }
    sum
}
fn part2(data: &Field) -> usize {
    let mut basin_map: HashMap<(usize, usize), usize> = HashMap::new();
    let mut basins: Vec<Vec<(usize, usize)>> = Vec::new();
    // Basin 0 is the 9s
    basins.push(vec![]);

    for y in 0..data.height {
        for x in 0..data.width {
            if basin_map.contains_key(&(x, y)) {
                continue;
            }
            if data.get(x, y) == 9 {
                basins[0].push((x, y));
                basin_map.insert((x, y), 0);
                continue;
            }
            // We're non-nine so check neighbours.
            let (value_above, basin_above) = if y > 0 {
                (data.get(x, y-1), *basin_map.get(&(x, y-1)).unwrap())
            } else {
                (9, 0)
            };
            let (value_left, basin_left) = if x > 0 {
                (data.get(x-1, y), *basin_map.get(&(x-1, y)).unwrap())
            } else {
                (9, 0)
            };
            // To start, assume a new basin (but don't create it yet).
            let mut basin = basins.len();
            if value_above != 9 {
                // Ok, join the basin above
                basin = basin_above;
            }
            if value_left != 9 {
                // Merge left basin into basin above
                if basin == basin_above {
                    let mut current_left_members = Vec::new();
                    std::mem::swap(basins.get_mut(basin_left).unwrap(), &mut current_left_members);
                    let new_basin = basins.get_mut(basin_above).unwrap();
                    for (xx, yy) in current_left_members {
                        new_basin.push((xx, yy));
                        *basin_map.get_mut(&(xx, yy)).unwrap() = basin_above;
                    }
                    new_basin.push((x, y));
                    basin_map.insert((x, y), basin_above);
                } else {
                    // Just add to left basin - above is 9.
                    basins.get_mut(basin_left).unwrap().push((x, y));
                    basin_map.insert((x, y), basin_left);
                }
            } else {
                // Use the above value but no merging
                if basin == basins.len() {
                    basins.push(vec![(x, y)]);
                } else {
                    basins.get_mut(basin).unwrap().push((x, y));
                }
                basin_map.insert((x, y), basin);
            }
        }
    }
    #[cfg(test)]
    for y in 0..data.height {
        for x in 0..data.width {
            print!("{}", *basin_map.get(&(x, y)).unwrap());
        }
        println!("");
    }
    let mut sizes = basins[1..].into_iter()
                               .map(Vec::len)
                               .collect::<Vec<usize>>();
    sizes.sort_by(|a, b| b.cmp(a));
    sizes[0] * sizes[1] * sizes[2]
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
    assert_eq!(part2(&data), 1134);
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
