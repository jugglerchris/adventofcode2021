use adventofcode2021::get_input;

fn find_error(s: &str) -> Option<char>
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
                    return Some(c);
                }
            }
            _ => panic!("Unknown character {}", c),
        }
    }
    None
}

fn part1(data: &[&str]) -> usize {
    let mut score = 0;
    for line in data {
        match find_error(line) {
            None => (),
            Some(')') => { score += 3; }
            Some(']') => { score += 57; }
            Some('}') => { score += 1197; }
            Some('>') => { score += 25137; }
            x => { panic!("{:?}", x); }
        }
    }
    return score;
}
fn part2(data: &[&str]) -> usize {
    unimplemented!()
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
    assert_eq!(part2(&data), 0);
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
