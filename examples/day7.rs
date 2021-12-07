use adventofcode2021::get_input;

type Coord = isize;

fn part1(positions: &[Coord]) -> isize {
    let min = positions.iter().cloned().min().unwrap();
    let max = positions.iter().cloned().max().unwrap();

    (min..=max).map(|v| {
        positions.iter()
                 .map(|p| (*p - v).abs())
                 .sum()
    }).min().unwrap()
}

#[test]
fn test() {
    let positions: Vec<Coord> = "16,1,2,0,4,2,7,1,2,14"
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    assert_eq!(part1(&positions), 37);
}

fn main() -> std::io::Result<()>{
    let input = get_input(7)?;

    let positions: Vec<Coord> = input.trim()
                                .split(',')
                                .map(|s| s.parse().unwrap())
                                .collect();

    // Part 1
    println!("{}", &part1(&positions));

    // Part 2
    /*
    println!("{}", &part2(positions));
    */

    Ok(())
}
