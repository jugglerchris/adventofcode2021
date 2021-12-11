use adventofcode2021::{get_input,parse_lines};

fn part1(data: &[u32]) -> usize {
    unimplemented!()
}
fn part2(data: &[u32]) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let tests = r#""#;
    let data: Vec<u32> = parse_lines(&tests);

    assert_eq!(part1(&data), 0);
    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input($N)?;

    let data: Vec<u32> = parse_lines(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
