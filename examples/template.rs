use adventofcode2021::{get_input,parse_lines};

type Data = ();
fn parse_input(input: &str) -> Data {
}

fn part1(data: &Data) -> usize {
    unimplemented!()
}
fn part2(data: &Data) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let tests = r#""#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 0);
    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input($N)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
