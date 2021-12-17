use adventofcode2021::{get_input,regex_parser};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct TargetArea {
    x0: isize,
    x1: isize,
    y0: isize,
    y1: isize,
}

regex_parser!(parse_area: TargetArea {
    RE = r#"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)"# =>
        |x0: isize, x1: isize, y0: isize, y1: isize| TargetArea { x0, x1, y0, y1 }
});

fn part1(area: &TargetArea) -> isize {
    // Map from valid x velocities to list of time steps within the target area.
    let mut valid_vx: Vec<isize> = Vec::new();

    for vx_i in 1..=(area.x1+1) {
        let mut x = 0;
        let mut vx = vx_i;
        loop {
            x += vx;
            if x > area.x1 {
                // Gone past
                break;
            } else if x >= area.x0 {
                valid_vx.push(vx_i);
                break;
            }
            vx -= 1;
            if vx <= 0 {
                // Stopped moving forwards.
                break;
            }
        }
    }

    let mut overall_max_y = 0;
    for vy_i in 1..(area.y0.abs()+1) {
        let mut max_y = 0;
        let mut vy = vy_i;
        let mut vx_x: Vec<(isize, isize)> = valid_vx.iter().map(|vx| (*vx, 0)).collect();
        let mut y = 0;
        while !vx_x.is_empty() {
            y += vy;
            vy -= 1;
            max_y = max_y.max(y);
            let in_y = if y < area.y0 {
                break;
            } else if y <= area.y1 {
                true
            } else {
                false
            };
            let mut in_x = false;
            let mut new_vx_x = Vec::new();
            for (mut vx, mut x) in vx_x {
                x += vx;
                if x > area.x1 {
                    // Gone past
                    continue;
                } else if x >= area.x0 {
                    in_x = true;
                }
                if vx >= 1 {
                    vx -= 1;
                }
                new_vx_x.push((vx, x));
            }
            vx_x = new_vx_x;
            if in_x && in_y {
                overall_max_y = overall_max_y.max(max_y);
            }
        }
    }
    overall_max_y
}
fn part2(area: &TargetArea) -> isize {
    unimplemented!()
}

#[test]
fn test() {
    let input = r#"target area: x=20..30, y=-10..-5"#;
    let area: TargetArea = <TargetArea as FromStr>::from_str(input).unwrap();

    assert_eq!(part1(&area), 45);
    assert_eq!(part2(&area), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(17)?;

    let area: TargetArea = <TargetArea as FromStr>::from_str(&input).unwrap();

    // Part 1
    println!("{}", part1(&area));

    // Part 2
    println!("{}", part2(&area));

    Ok(())
}
