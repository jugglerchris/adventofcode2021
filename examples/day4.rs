use adventofcode2021::{get_input};

type Number = usize;

#[derive(Clone, Debug)]
struct BingoCard {
    data: [Number; 25],
    row_remain: [u8; 5],
    col_remain: [u8; 5],
}

impl BingoCard {
    pub fn from(ns: &[Number]) -> BingoCard {
        BingoCard {
            data: ns.try_into().unwrap(),
            row_remain: [5; 5],
            col_remain: [5; 5],
        }
    }

    pub fn call(&mut self, number: Number) -> bool {
        match self.data
            .iter()
            .enumerate()
            .find(|(_i, n)| **n == number)
        {
            None => false,
            Some((i, n)) => {
                let row = i/5;
                let col = i%5;
                self.data[i] = 0;
                self.row_remain[row] -= 1;
                self.col_remain[col] -= 1;
                self.row_remain[row] == 0 || self.col_remain[col] == 0
            }
        }
    }

    pub fn score(&self) -> usize {
        self.data.iter().sum()
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
    let card_numbers: Vec<Number> = remain
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    assert_eq!(card_numbers.len() % 25, 0);
    let cards = card_numbers.chunks(25)
        .map(BingoCard::from)
        .collect::<Vec<_>>();

    // Part 1
    {
        let mut cards = cards.clone();
        'outer: for n in &numbers_called {
            for card in &mut cards {
                if card.call(*n) {
                    println!("Card called with score {}",
                             card.score() * n);
                    break 'outer;
                }
            }
        }
    }

    // Part 2

    Ok(())
}
