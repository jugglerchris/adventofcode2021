use adventofcode2021::{get_input,parse_lines,regex_parser};

#[derive(Copy, Clone, Debug)]
struct Data {
    pos1: usize,
    pos2: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Pos(usize);
regex_parser!(parse_pos: Pos {
    POS = r#"^Player [12] starting position: (\d+)$"# => |pos: usize| Pos(pos)
});

fn parse_input(input: &str) -> Data {
    let positions: Vec<Pos> = parse_lines(input);
    assert_eq!(positions.len(), 2);
    Data {
        pos1: positions[0].0,
        pos2: positions[1].0,
    }
}

fn part1(data: &Data) -> usize {
    let mut pos1 = data.pos1;
    let mut pos2 = data.pos2;
    let mut score1 = 0;
    let mut score2 = 0;
    let mut die = 1;
    let mut throws = 0;
    while score2 < 1000 {
        for _ in 0..3 {
            pos1 += die;
            die += 1;
            throws += 1;
            if die > 100 {
                die -= 100;
            }
        }
        while pos1 > 10 {
            pos1 -= 10;
        }
        score1 += pos1;
        std::mem::swap(&mut pos1, &mut pos2);
        std::mem::swap(&mut score1, &mut score2);
    }
    throws * score1
}

/*
struct States {
    counts: Vec<usize>,
    player1_wins: usize,
    player2_wins: usize,
}

impl States {
    pub fn new() -> States {
        let counts = vec![0; 10*10*21*21];
        States {
            counts: counts,
            player1_wins: 0,
            player2_wins: 0,
        }
    }
    pub fn new_with_pos(pos1: usize, pos2: usize) -> States {
        let mut result = States::new();
        result.add_state(pos1, pos2, 0, 0, 1);
        result
    }

    pub fn add_state(&mut self, pos1: usize, pos2: usize, score1: usize, score2: usize, count: usize) {
        self.counts[(pos1-1) + (pos2-1)*10 + score1*10*10 + score2*10*10*21] += count;
    }
    pub fn get_state_count(&self, pos1: usize, pos2: usize, score1: usize, score2: usize) -> usize {
        self.counts[(pos1-1) + (pos2-1)*10 + score1*10*10 + score2*10*10*21]
    }
    pub fn add_wins(&mut self, count: usize, player2: bool) {
        if player2 {
            self.player2_wins += count;
        } else {
            self.player1_wins += count;
        }
    }
    pub fn wins(&self) -> (usize, usize) {
        (self.player1_wins, self.player2_wins)
    }
    pub fn states_left(&self) -> usize {
        self.counts.iter().cloned().sum()
    }
    pub fn max_states_left(&self) -> usize {
        self.counts.iter().cloned().max().unwrap()
    }
    pub fn print_stats(&self) {
        let mut scores1 = [0usize;21];
        let mut scores2 = [0usize;21];
        let mut positions1 = [0usize;10];
        let mut positions2 = [0usize;10];
        for pos1 in 1..=10 {
            for pos2 in 1..=10 {
                for score1 in 0..21 {
                    for score2 in 0..21 {
                        let count = self.get_state_count(pos1, pos2, score1, score2);
                        scores1[score1] += count;
                        scores2[score2] += count;
                        positions1[pos1-1] += count;
                        positions2[pos2-1] += count;
                    }
                }
            }
        }
        dbg!(scores1);
        dbg!(scores2);
        dbg!(positions1);
        dbg!(positions2);
    }
}
*/

struct States1 {
    counts: Vec<usize>,
    wins: Vec<usize>,
}
impl States1 {
    pub fn new() -> States1 {
        let counts = vec![0; 10*21];
        let wins = vec![];
        States1 { counts, wins }
    }
    pub fn new_with_pos(pos: usize) -> States1 {
        let mut result = States1::new();
        result.add_state(pos, 0, 1);
        result
    }

    pub fn add_state(&mut self, pos: usize, score: usize, count: usize) {
        self.counts[(pos-1) + score*10] += count;
    }
    pub fn get_state_count(&self, pos: usize, score: usize) -> usize {
        self.counts[(pos-1) + score*10]
    }
    pub fn add_wins(&mut self, turn: usize, count: usize) {
        while self.wins.len() <= turn {
            self.wins.push(0);
        }
        self.wins[turn] += count;
    }
    pub fn wins(&self) -> Vec<usize> {
        self.wins.clone()
    }
    pub fn states_left(&self) -> usize {
        self.counts.iter().cloned().sum()
    }
    pub fn max_states_left(&self) -> usize {
        self.counts.iter().cloned().max().unwrap()
    }
    pub fn print_stats(&self) {
        dbg!(&self.counts);
        dbg!(&self.wins);
    }
}

fn part2(data: &Data) -> usize {
    let mut moves = [0usize; 10];
    for a in 1..=3 {
        for b in 1..=3 {
            for c in 1..=3 {
                moves[a+b+c] += 1;
            }
        }
    }

    let mut state1 = States1::new_with_pos(data.pos1);
    let mut state2 = States1::new_with_pos(data.pos2);
    let mut turn = 0;
    while state1.max_states_left() > 0 || state2.max_states_left() > 0 {
        #[cfg(test)] dbg!(state1.max_states_left());
        #[cfg(test)] dbg!(state2.max_states_left());
        #[cfg(test)] state1.print_stats();
        #[cfg(test)] state2.print_stats();

        let mut newstate1 = States1::new();
        std::mem::swap(&mut newstate1.wins, &mut state1.wins);
        let mut newstate2 = States1::new();
        std::mem::swap(&mut newstate2.wins, &mut state2.wins);
        for pos in 1..=10 {
            for score in 0..21 {
                let count1 = state1.get_state_count(pos, score);
                let count2 = state2.get_state_count(pos, score);
                for jump in 3..=9 {
                    let mut newpos = pos + jump;
                    if newpos > 10 {
                        newpos -= 10;
                    }
                    let newscore = score + newpos;
                    if newscore >= 21 {
                        newstate1.add_wins(turn, count1 * moves[jump]);
                        newstate2.add_wins(turn, count2 * moves[jump]);
                    } else {
                        newstate1.add_state(newpos, newscore, count1 * moves[jump]);
                        newstate2.add_state(newpos, newscore, count2 * moves[jump]);
                    }
                }
            }
        }

        turn += 1;
        state1 = newstate1;
        state2 = newstate2;
    }
    let mut tot_wins1 = 0;
    let mut tot_wins2 = 0;
    let wins1 = state1.wins();
    let wins2 = state2.wins();

    let mut states_p1 = 1;
    let mut states_p2 = 1;
    for (t, (w1, w2)) in wins1.into_iter().zip(wins2.into_iter()).enumerate() {
        tot_wins1 += w1 * states_p2;
        states_p1 = states_p1 * 27 - w1;

        tot_wins2 += w2 * states_p1;
        states_p2 = states_p2 * 27 - w2;
    }
    tot_wins1.max(tot_wins2)
}

#[test]
fn test() {
    let tests = r#"Player 1 starting position: 4
Player 2 starting position: 8
"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 739785);
    assert_eq!(part2(&data), 444356092776315);
}

fn main() -> std::io::Result<()>{
    let input = get_input(21)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
