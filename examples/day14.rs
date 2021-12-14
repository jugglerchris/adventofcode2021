use adventofcode2021::{get_input,parse_lines,regex_parser};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Rule {
    left: u8,
    right: u8,
    new: u8,
}

regex_parser!(parse_rule: Rule {
    RULE = r#"^(.)(.) -> (.)$"# => |left: char, right: char, new: char| {
        let left = left as u8;
        let right = right as u8;
        let new = new as u8;
        Rule {
        left,
        right,
        new
    }}
});

fn parse_input(input: &str) -> (String, Vec<Rule>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(parts.len(), 2);
    let template = parts[0].trim();
    let rules: Vec<Rule> = parse_lines(parts[1]);
    (template.into(), rules)
}

fn step(template: &[u8], rules: &HashMap<(u8, u8), u8>) -> Vec<u8>
{
    let mut result: Vec<u8> = vec![template[0]];
    for window in template.windows(2) {
        if let Some(n) = rules.get(&(window[0], window[1])) {
            result.push(*n);
        }
        result.push(window[1]);
    }
    result
}

fn part1(template: &str, rules: &[Rule]) -> usize {
    let rules: HashMap<(u8, u8), u8> =
        rules.into_iter()
             .map(|Rule { left, right, new }| ((*left, *right), *new))
             .collect();

    let mut s: Vec<u8> = template.as_bytes().into();
    for _ in 0..10 {
        s = step(&s, &rules);
    }
    let mut counts = vec![0usize; 26];
    for c in s {
        match c {
            b'A'..=b'Z' => {
                counts[c as usize - (b'A' as usize)] += 1;
            }
            _ => panic!(),
        }
    }
    let min = counts.iter().filter(|&count| *count != 0).min().unwrap();
    let max = counts.iter().filter(|&count| *count != 0).max().unwrap();
    max - min
}
fn part2(template: &str, rules: &[Rule]) -> usize {
    let rules: HashMap<(u8, u8), u8> =
        rules.into_iter()
             .map(|Rule { left, right, new }| ((*left, *right), *new))
             .collect();

    let mut counts = vec![0usize; 26];
    // Count hte final char
    let s = template.as_bytes().iter().cloned().collect::<Vec<u8>>();
    counts[s[s.len() - 1] as usize - (b'A' as usize)] = 1;
    for w in s.windows(2) {
        let mut s: Vec<u8> = w.iter().cloned().collect::<Vec<u8>>();
        for _ in 0..40 {
            s = step(&s, &rules);
        }
        s.pop().unwrap(); // Don't double count the edge one
        for c in s {
            match c {
                b'A'..=b'Z' => {
                    counts[c as usize - (b'A' as usize)] += 1;
                }
                _ => panic!(),
            }
        }
    }
    let min = counts.iter().filter(|&count| *count != 0).min().unwrap();
    let max = counts.iter().filter(|&count| *count != 0).max().unwrap();
    max - min
}

#[test]
fn test() {
    let tests = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;
    let (template, rules) = parse_input(&tests);

    assert_eq!(part1(&template, &rules), 1588);
    assert_eq!(part2(&template, &rules), 2188189693529);
}

fn main() -> std::io::Result<()>{
    let input = get_input(14)?;

    let (template, rules) = parse_input(&input);

    // Part 1
    println!("{}", part1(&template, &rules));

    // Part 2
    println!("{}", part2(&template, &rules));

    Ok(())
}
