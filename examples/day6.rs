use adventofcode2021::get_input;

type Timer = u8;

fn part1(timers: &[Timer], days: usize) -> usize {
    let mut fish: Vec<u8> = timers.iter().cloned().collect();
    for _ in 0..days {
        let mut new_fish = 0;
        for f in &mut fish {
            if *f == 0 {
                *f = 6;
                new_fish += 1;
            } else {
                *f -= 1;
            }
        }
        for _ in 0..new_fish {
            fish.push(8);
        }
    }
    fish.len()
}

#[test]
fn test()
{
    assert_eq!(part1(&[3,4,3,1,2], 80), 5934);
    assert_eq!(part2(&[3,4,3,1,2], 256), 26984457539);
}
fn part2(timers: &[Timer], days: usize) -> usize {
    let mut fish_counts = vec![0usize; 9];
    for &t in timers {
        fish_counts[t as usize] += 1;
    }
    for _ in 0..days {
        let mut zeroes = fish_counts[0];
        for i in 1..9 {
            fish_counts[i-1] = fish_counts[i];
        }
        fish_counts[6] += zeroes;
        fish_counts[8] = zeroes;
    }
    fish_counts.iter().sum()
}

fn main() -> std::io::Result<()>{
    let input = get_input(6)?;

    let timers: Vec<Timer> = input.trim()
                                .split(',')
                                .map(|s| s.parse().unwrap())
                                .collect();

    // Part 1
    println!("{}", part1(&timers, 80));

    // Part 2
    println!("{}", part2(&timers, 256));

    Ok(())
}
