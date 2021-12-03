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
    usize::from_str_radix(remaining[0], 2).unwrap()
}

fn get_ox_gen_rating(data: &[String]) -> usize {
    reduce_using(data,
        |ones, total| {
            if ones*2 >= total { 1 } else { 0 }
        })
}
fn get_co2_scrubber_rating(data: &[String]) -> usize {
    reduce_using(data,
        |ones, total| {
            if ones*2 >= total { 0 } else { 1 }
        })
}

#[test]
fn test_reduce_using() {
    let test_data: Vec<String> = vec![
        "00100".into(),
        "11110".into(),
        "10110".into(),
        "10111".into(),
        "10101".into(),
        "01111".into(),
        "00111".into(),
        "11100".into(),
        "10000".into(),
        "11001".into(),
        "00010".into(),
        "01010".into(),
    ];
    assert_eq!(get_ox_gen_rating(&test_data), 23);
    assert_eq!(get_co2_scrubber_rating(&test_data), 10);
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
    let ox_gen_rating = get_ox_gen_rating(&data);
    let co2_scrubber_rating = get_co2_scrubber_rating(&data);
    println!("{}", ox_gen_rating*co2_scrubber_rating);

    Ok(())
}
