use adventofcode2021::{get_input,parse_lines,regex_parser};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Point(isize, isize);

#[derive(Debug, Clone, Copy)]
pub enum Fold {
    Vert {
        x: isize,
    },
    Horiz {
        y: isize,
    }
}

regex_parser!(parse_point: Point {
    PT = r#"^(\d+),(\d+)$"# => |x: isize, y: isize| Point(x, y)
});

regex_parser!(parse_fold: Fold {
    VERT = r#"^fold along x=(\d+)$"# => |x: isize| Fold::Vert{x},
    HORIZ = r#"^fold along y=(\d+)$"# => |y: isize| Fold::Horiz{y}
});

fn do_fold(points: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    let mut results = HashSet::new();

    for pt in points {
        let newpt = match fold {
            Fold::Vert{x} => {
                assert!(pt.0 != *x);
                let newx = if pt.0 > *x {
                    2*(*x) - pt.0
                } else {
                    pt.0
                };
                Point(newx, pt.1)
            }
            Fold::Horiz{y} => {
                assert!(pt.1 != *y);
                let newy = if pt.1 > *y {
                    2*(*y) - pt.1
                } else {
                    pt.1
                };
                Point(pt.0, newy)
            }
        };
        results.insert(newpt);
    }

    results
}

#[cfg(test)]
fn draw_paper(points: &HashSet<Point>) {
    let mut points = points.iter().cloned().collect::<Vec<Point>>();
    points.sort_by_key(|&Point(x, y)| (y, x));
    let mut x = 0;
    let mut y = 0;
    for Point(px, py) in points {
        while py > y {
            println!("");
            y += 1;
            x = 0;
        }
        while px > x {
            print!(" ");
            x += 1;
        }
        print!("#");
        x += 1;
    }
}

fn part1(points: &[Point], folds: &[Fold]) -> usize {
    let points: HashSet<Point> = points.iter().cloned().collect();
    #[cfg(test)]
    draw_paper(&points);

    let newpoints = do_fold(&points, &folds[0]);
    #[cfg(test)]
    draw_paper(&newpoints);
    newpoints.len()
}
fn part2(points: &[Point], folds: &[Fold]) -> usize {
    unimplemented!()
}



#[test]
fn test() {
    let input = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);
    let points: Vec<Point> = parse_lines(parts[0]);
    let folds: Vec<Fold> = parse_lines(parts[1]);

    assert_eq!(part1(&points, &folds), 17);
    assert_eq!(part2(&points, &folds), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(13)?;

    let parts = input.split("\n\n").collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);
    let points: Vec<Point> = parse_lines(parts[0]);
    let folds: Vec<Fold> = parse_lines(parts[1]);

    // Part 1
    println!("{}", part1(&points, &folds));

    // Part 2
    println!("{}", part2(&points, &folds));

    Ok(())
}
