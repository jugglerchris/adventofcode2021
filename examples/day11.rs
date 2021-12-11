use adventofcode2021::get_input;

#[derive(Debug, Clone)]
struct Grid {
    octopi: [u8; 100],
    pub flashes: usize,
}

impl Grid {
    pub fn from(s: &str) -> Grid {
        let mut octopi = [0; 100];
        let mut i = 0;
        for c in s.chars() {
            match c {
                '0'..='9' => {
                    octopi[i] = (c as u8) - b'0';
                    i += 1;
                }
                _ => ()
            }
        }
        assert_eq!(i, 100);
        Grid { octopi, flashes: 0 }
    }

    pub fn step(&mut self) {
        for oct in &mut self.octopi {
            *oct += 1;
        }
        let mut flash_points = Vec::new();
        for y in 0..10 {
            for x in 0..10 {
                if self.octopi[10*y + x] == 10 {
                    flash_points.push((x, y));
                    self.flashes += 1;
                }
            }
        }
        while let Some((x, y)) = flash_points.pop() {
            let yymin = y.max(1) - 1;
            let yymax = (y+1).min(9);
            let xxmin = x.max(1) - 1;
            let xxmax = (x+1).min(9);
            for yy in yymin..=yymax {
                for xx in xxmin..=xxmax {
                    if x==xx && y==yy { continue }
                    let o = &mut self.octopi[yy*10 + xx];
                    if *o == 9 {
                        // New flash
                        flash_points.push((xx, yy));
                        self.flashes += 1;
                    }
                    if *o != 10 {
                        *o += 1;
                    }
                }
            }
        }
        for o in &mut self.octopi {
            assert!(*o <= 10);
            if *o == 10 {
                *o = 0;
            }
        }
    }

    pub fn print(&self) {
        for y in 0..10 {
            for x in 0..10 {
                print!("{}", self.octopi[y*10+x]);
            }
            println!("");
        }
        println!("");
    }
}

fn part1(data: &Grid) -> usize {
    let mut grid = data.clone();
    for _ in 0..100 {
        grid.step();
        #[cfg(test)]
        grid.print();
    }
    grid.flashes
}
fn part2(data: &Grid) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let tests = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
    let data = Grid::from(tests);

    assert_eq!(part1(&data), 1656);
    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(11)?;

    let data = Grid::from(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
