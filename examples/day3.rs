use adventofcode2021::{get_input,parse_lines};
fn main() -> std::io::Result<()>{
    let input = get_input(3)?;

    let data: Vec<String> = parse_lines(&input);

    // Part 1
    let mut column_sums = vec![0usize; data[0].len()];
    for s in &data {
        for (i, c) in s.chars().enumerate() {
            if c == '1' {
                column_sums[i] += 1;
            }
        }
    }
    let num_values = data.len();
    let mut gamma = 0usize;
    let mut epsilon = 0usize;
    for s in column_sums {
        gamma <<= 1;
        epsilon <<= 1;
        if s > (num_values/2) {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }
    println!("{}", gamma*epsilon);

    // Part 2
    /*
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
