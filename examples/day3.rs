use adventofcode2021::{get_input,parse_lines};

fn reduce_using<F:Fn(usize, usize) -> usize>(data: &[String], keep_digit: F) -> usize {
    let mut col = 0;
    let mut remaining = data.iter()
        .map(|s| &s[..])
        .collect::<Vec<&str>>();

    while remaining.len() > 1 {
        let mut zeros = Vec::new();
        let mut ones = Vec::new();
        for s in remaining {
            if s.as_bytes()[col] == b'1' {
                ones.push(s);
            } else {
                zeros.push(s);
            }
        }
        remaining = match keep_digit(ones.len(), ones.len()+zeros.len()) {
            0 => zeros,
            1 => ones,
            _ => unreachable!(),
        };
        col += 1;
    }
    assert_eq!(remaining.len(), 1);
    return usize::from_str_radix(remaining[0], 2).unwrap();
}

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
    let ox_gen_rating = reduce_using(
        &data,
        |ones, total| {
            if ones*2 >= total { 1 } else { 0 }
        });
    let co2_scrubber_rating = reduce_using(
        &data,
        |ones, total| {
            if ones*2 <= total { 0 } else { 1 }
        });
    println!("{}", ox_gen_rating*co2_scrubber_rating);

    Ok(())
}
