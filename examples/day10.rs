use adventofcode2021::get_input;

#[derive(Debug)]
enum ParseResult {
    Error(char),
    Incomplete(Vec<char>),
}

use ParseResult::*;

fn find_error(s: &str) -> ParseResult
{
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '[' => stack.push(']'),
            '(' => stack.push(')'),
            '<' => stack.push('>'),
            '{' => stack.push('}'),
            ']' | ')' | '>' | '}' => {
                let top = stack.pop().unwrap();
                if top != c {
                    return Error(c);
                }
            }
            _ => panic!("Unknown character {}", c),
        }
    }
    Incomplete(stack)
}

fn part1(data: &[&str]) -> usize {
    let mut score = 0;
    for line in data {
        match find_error(line) {
            Incomplete(_) => (),
            Error(')') => { score += 3; }
            Error(']') => { score += 57; }
            Error('}') => { score += 1197; }
            Error('>') => { score += 25137; }
            x => { panic!("{:?}", x); }
        }
    }
    return score;
}
fn part2(data: &[&str]) -> usize {
    let mut scores = Vec::new();
    for line in data {
        match find_error(line) {
            Error(_) => (),
            Incomplete(mut v) => {
                let mut score = 0usize;
                while let Some(c) = v.pop() {
                    match c {
                        ')' => {
                            score = score * 5 + 1;
                        }
                        ']' => {
                            score = score * 5 + 2;
                        }
                        '}' => {
                            score = score * 5 + 3;
                        }
                        '>' => {
                            score = score * 5 + 4;
                        }
                        _ => panic!()
                    }
                }
                scores.push(score);
            }
        }
    }
    scores.sort();
    assert!(scores.len() & 1 == 1);
    scores[scores.len() / 2]
}

#[test]
fn test() {
    let tests = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#;
    let data: Vec<&str> = tests.lines().collect();

    assert_eq!(part1(&data), 26397);
    assert_eq!(part2(&data), 288957);
}

fn main() -> std::io::Result<()>{
    let input = get_input(10)?;

    let data: Vec<&str> = input.lines().collect();

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
