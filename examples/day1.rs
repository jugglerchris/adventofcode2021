use adventofcode2021::{get_input,parse_lines};
fn main() -> std::io::Result<()>{
    let input = get_input(1)?;

    let depths: Vec<u32> = parse_lines(&input);

    // Part 1
    println!("{}", depths.windows(2)
                         .map(|w| if w[1]>w[0] { 1 } else { 0 })
                         .sum::<u32>());

    // Part 2
    println!("{}", depths.windows(3)
                             .map(|w| w.iter().sum::<u32>())
                             .collect::<Vec<_>>()
                             .windows(2)
                             .map(|w| if w[1]>w[0] { 1 } else { 0 })
                             .sum::<u32>());

    Ok(())
}
