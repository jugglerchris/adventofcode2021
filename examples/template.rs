use adventofcode2019::{get_input,parse_lines};
fn main() -> std::io::Result<()>{
    let input = get_input(1)?;

    let increments: Vec<u32> = parse_lines(&input);

    // Part 1
    println!("{}", increments.iter()
                             .map(|mass| mass/3 - 2)
                             .sum::<u32>());

    // Part 2
    println!("{}", increments.iter()
                             .map(|mass| {
                                 let mut mass = *mass;
                                 let mut fuel = 0;
                                 while mass > 5 {
                                     let f = mass/3 - 2;
                                     fuel += f;
                                     mass = f;
                                 }
                                 fuel
                              })
                             .sum::<u32>());

    Ok(())
}