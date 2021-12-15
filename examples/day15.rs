use adventofcode2021::{get_input};
use std::collections::BinaryHeap;

fn part1(data: &[Vec<u8>]) -> isize {
    let dest_x = data[0].len()-1;
    let dest_y = data.len()-1;
    let mut risks = vec![];
    for _ in 0..=dest_y {
        risks.push(vec![isize::MAX; dest_x+1]);
    }

    let mut next_squares = BinaryHeap::new();
    next_squares.push((-0, 0, 0)); // destination risk, x, y
    
    while let Some((risk, x, y)) = next_squares.pop() {
        #[cfg(test)]
        dbg!((-risk, x, y));
        let risk = -risk; // The heap has inverted risk as it's a max-heap
        if y == dest_y && x == dest_x {
            return risk;
        }
        if risk >= risks[y][x] {
            continue;  // No need to keep looking here
        }
        risks[y][x] = risk;
        let mut try_step = |new_x, new_y: usize| {
            let new_risk = risk + (data[new_y][new_x] as isize);
            if new_risk < risks[new_y][new_x] {
                next_squares.push((-new_risk, new_x, new_y));
            }
        };
        if y > 0 {
            try_step(x, y-1);
        }
        if x > 0 {
            try_step(x-1, y);
        }
        if y <= dest_y-1 {
            try_step(x, y+1);
        }
        if x <= dest_x-1 {
            try_step(x+1, y);
        }
    }
    panic!()
}
fn part2(_data: &[Vec<u8>]) -> usize {
    unimplemented!()
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let data: Vec<Vec<u8>> = input.split_whitespace()
                                  .map(|s| s.as_bytes()
                                            .iter()
                                            .map(|c| *c - b'0')
                                            .collect())
                                  .collect();
    data
}

#[test]
fn test() {
    let input = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;
    let data = parse_input(&input);

    assert_eq!(part1(&data), 40);
    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(15)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
