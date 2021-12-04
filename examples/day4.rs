use adventofcode2021::{get_input};

type Number = u8;

struct BingoCard {
    data: [Number; 25],
}

impl BingoCard {
    pub fn from(ns: &[u8]) -> BingoCard {
        BingoCard {
            data: ns.try_into().unwrap(),
        }
    }
}

fn main() -> std::io::Result<()>{
    let input = get_input(4)?;

    let lines = input.lines().collect::<Vec<&str>>();

    let numbers_called: Vec<usize> = lines[0].split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    assert_eq!(lines[1], "");
    let remain = &input[lines[0].len()..];
    let card_numbers: Vec<u8> = remain
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    assert_eq!(card_numbers.len() % 25, 0);
    let cards = card_numbers.chunks(25)
        .map(BingoCard::from)
        .collect::<Vec<_>>();

    // Part 1
    println!("Got {} numbers and {} cards", numbers_called.len(), cards.len());

    // Part 2

    Ok(())
}
