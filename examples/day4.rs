use adventofcode2021::{get_input};

type Number = usize;

#[derive(Clone, Debug)]
struct BingoCard {
    data: [Option<Number>; 25],
    row_remain: [u8; 5],
    col_remain: [u8; 5],
}

impl BingoCard {
    pub fn from(ns: &[Number]) -> BingoCard {
        let data = ns
            .iter()
            .map(|n| Some(*n))
            .collect::<Vec<Option<Number>>>();
        BingoCard {
            data: data.into_iter().collect::<Vec<_>>().try_into().unwrap(),
            row_remain: [5; 5],
            col_remain: [5; 5],
        }
    }

    pub fn call(&mut self, number: Number) -> bool {
        match self.data
            .iter()
            .enumerate()
            .find(|(_i, n)| {
                **n == Some(number)
            })
        {
            None => false,
            Some((i, _n)) => {
                let row = i/5;
                let col = i%5;
                self.data[i] = None;
                self.row_remain[row] -= 1;
                self.col_remain[col] -= 1;
                self.row_remain[row] == 0 || self.col_remain[col] == 0
            }
        }
    }

    pub fn score(&self) -> usize {
        self.data
            .iter()
            .filter_map(|s| *s)
            .sum()
    }
}

fn parse_input(input: &str) -> (Vec<Number>, Vec<BingoCard>) {
    let lines = input.lines().collect::<Vec<&str>>();

    let numbers_called: Vec<Number> = lines[0].split(',')
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

    (numbers_called, cards)
}

fn part1(numbers_called: &Vec<Number>, cards: &Vec<BingoCard>) -> usize {
    let mut cards = cards.clone();
    for n in numbers_called {
        for card in &mut cards {
            if card.call(*n) {
                return card.score() * n;
            }
        }
    }
    unreachable!()
}

fn part2(numbers_called: &Vec<Number>, cards: &Vec<BingoCard>) -> usize {
    let mut cards = cards.iter()
        .map(|c| Some(c.clone()))
        .collect::<Vec<_>>();
    let mut cards_left = cards.len();
    for n in numbers_called {
        for maybe_card in &mut cards {
            if let Some(card) = maybe_card {
                if card.call(*n) {
                    cards_left -= 1;
                    if cards_left == 0 {
                        return card.score() * n;
                    } else {
                        *maybe_card = None;
                    }
                }
            }
        }
    }
    unreachable!()
}

fn main() -> std::io::Result<()>{
    let input = get_input(4)?;

    let (numbers_called, cards) = parse_input(&input);

    // Part 1
    println!("Part 1: {}", part1(&numbers_called, &cards));

    // Part 2
    println!("Part 2: {}", part2(&numbers_called, &cards));

    Ok(())
}
