use adventofcode2021::{get_input,parse_lines};
fn main() -> std::io::Result<()>{
    let input = get_input(1)?;

    let depths: Vec<u32> = parse_lines(&input);

    // Part 1
    println!("{}", depths.windows(2)
                         .map(|w| if w[1]>w[0] { 1 } else { 0 })
                         .sum::<u32>());

    /*
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
                             */

    Ok(())
}
