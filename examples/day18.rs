use adventofcode2021::get_input;

type Numb = isize;

#[derive(Clone, Eq, PartialEq)]
enum SnailNumber {
    Number(Numb),
    Pair(Box<SnailNumber>, Box<SnailNumber>),
}

impl std::fmt::Debug for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailNumber::Number(nn) => {
                write!(f, "{}", nn)
            }
            SnailNumber::Pair(a, b) => {
                write!(f, "[{:?},{:?}]", &*a, &*b)
            }
        }
    }
}

impl SnailNumber {
    fn parse(s: &str) -> (SnailNumber, &str) {
        use SnailNumber::*;
        match s.as_bytes()[0] {
            b'[' => {
                let (a, remain) = SnailNumber::parse(&s[1..]);
                assert_eq!(remain.chars().next(), Some(','));
                let (b, remain) = SnailNumber::parse(&remain[1..]);
                assert_eq!(remain.chars().next(), Some(']'));
                (Pair(Box::new(a), Box::new(b)), &remain[1..])
            }
            b'0'..=b'9' => {
                let mut remain = s;
                let mut v = 0;
                while remain.len() > 0 {
                    let c = remain.as_bytes()[0];
                    match c {
                        b'0'..=b'9' => {
                            v = v*0 + (c - b'0') as Numb;
                            remain = &remain[1..];
                        }
                        _ => break,
                    }
                }
                (SnailNumber::Number(v), remain)
            }
            _ => { panic!() }
        }
    }
}

type Data = Vec<SnailNumber>;

fn parse_input(input: &str) -> Data {
    let mut v = Vec::new();
    for s in input.lines() {
        let (n, remain) = SnailNumber::parse(s);
        assert_eq!(remain.len(), 0);
        v.push(n);
    }
    v
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum ExplodeResult {
    Nothing, // No exploding to do
    Exploding {
        left: Option<Numb>,
        repl: SnailNumber,
        right: Option<Numb>,
    },
}

impl ExplodeResult {
    fn unwrap(self) -> SnailNumber {
        match self {
            ExplodeResult::Nothing => { panic!("Try to unrwap Nothing") }
            ExplodeResult::Exploding{ repl, .. } => repl
        }
    }
}

fn addleft(n: &SnailNumber, v: Numb) -> SnailNumber {
    use SnailNumber::*;
    match n {
        Number(nn) => Number(nn + v),
        Pair(a, b) => Pair(Box::new(addleft(a, v)), b.clone()),
    }
}

fn addright(n: &SnailNumber, v: Numb) -> SnailNumber {
    use SnailNumber::*;
    match n {
        Number(nn) => Number(nn + v),
        Pair(a, b) => Pair(a.clone(), Box::new(addright(b, v))),
    }
}

fn explode(n: &SnailNumber, depth: usize) -> ExplodeResult {
    use SnailNumber::*;
    match n {
        Number(_) => ExplodeResult::Nothing,
        Pair(a, b) => {
            if depth == 4 {
                if let (Number(aa), Number(bb)) = (&**a, &**b) {
                    let left = Some(*aa);
                    let right = Some(*bb);
                    let repl = Number(0);
                    return ExplodeResult::Exploding { left, repl, right };
                } else {
                    panic!("Expected pair of numbers");
                }
            } else {
                if let ExplodeResult::Exploding { left, repl, right } = explode(&a, depth+1) {
                    if let Some(right) = right {
                        ExplodeResult::Exploding {
                            left,
                            repl: Pair(Box::new(repl), Box::new(addleft(b, right))),
                            right: None
                        }
                    } else {
                        ExplodeResult::Exploding {
                            left,
                            repl: Pair(Box::new(repl), b.clone()),
                            right: None
                        }
                    }
                } else {
                    if let ExplodeResult::Exploding { left, repl, right } = explode(&b, depth+1) {
                        if let Some(left) = left {
                            ExplodeResult::Exploding {
                                left: None,
                                repl: Pair(Box::new(addright(a, left)), Box::new(repl)),
                                right
                            }
                        } else {
                            ExplodeResult::Exploding {
                                left: None,
                                repl: Pair(a.clone(), Box::new(repl)),
                                right
                            }
                        }
                    } else {
                        ExplodeResult::Nothing
                    }
                }
            }
        }
    }
}

#[test]
fn test_explode() {
    assert_eq!(explode(&SnailNumber::parse("[[[[[9,8],1],2],3],4]").0, 0).unwrap(),
                       SnailNumber::parse("[[[[0,9],2],3],4]").0);
    assert_eq!(explode(&SnailNumber::parse("[7,[6,[5,[4,[3,2]]]]]").0, 0).unwrap(),
                       SnailNumber::parse("[7,[6,[5,[7,0]]]]").0);
    assert_eq!(explode(&SnailNumber::parse("[[6,[5,[4,[3,2]]]],1]").0, 0).unwrap(),
                       SnailNumber::parse("[[6,[5,[7,0]]],3]").0);
    assert_eq!(explode(&SnailNumber::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").0, 0).unwrap(),
                       SnailNumber::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").0);
    assert_eq!(explode(&SnailNumber::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").0, 0).unwrap(),
                       SnailNumber::parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]").0);
}

enum SplitResult {
    Nothing,
    Replace(SnailNumber),
}

fn split(n: &SnailNumber) -> SplitResult {
    use SnailNumber::*;
    match n {
        Number(n) => {
            if *n >= 10 {
                SplitResult::Replace(
                    Pair(Box::new(Number(*n/2)), Box::new(Number((*n+1)/2))))
            } else {
                SplitResult::Nothing
            }
        }
        Pair(a, b) => {
            if let SplitResult::Replace(repl) = split(a) {
                SplitResult::Replace(
                    Pair(Box::new(repl), b.clone()))
            } else if let SplitResult::Replace(repl) = split(b) {
                SplitResult::Replace(
                    Pair(a.clone(), Box::new(repl)))

            } else {
                SplitResult::Nothing
            }
        }
    }
}

fn reduce(mut n: SnailNumber) -> SnailNumber {
    #[cfg(test)]
    dbg!((&n, "Reducing"));
    loop {
        if let ExplodeResult::Exploding{left: _left, repl, right: _right} = explode(&n, 0) {
            n = repl;
            #[cfg(test)]
            dbg!((&n, "Exploded"));
            continue;
        }
        if let SplitResult::Replace(new) = split(&n) {
            n = new;
            #[cfg(test)]
            dbg!((&n, "Split"));
            continue;
        }
        // Nothing to do
        break;
    }
    n
}

fn magnitude(n: &SnailNumber) -> Numb {
    use SnailNumber::*;
    match n {
        Number(nn) => *nn,
        Pair(a, b) => {
            3*magnitude(a) + 2*magnitude(b)
        }
    }
}

fn part1(data: &Data) -> Numb {
    use SnailNumber::*;
    let mut sum = data[0].clone();
    for n in data[1..].iter() {
        #[cfg(test)]
        dbg!(&sum);
        sum = Pair(Box::new(sum), Box::new(n.clone()));
        #[cfg(test)]
        dbg!((&sum, "added"));
        sum = reduce(sum);
        #[cfg(test)]
        dbg!((&sum, "reduced"));
    }
    magnitude(&sum)
}
fn part2(_data: &Data) -> Numb {
    unimplemented!()
}

#[test]
fn test() {
    let test_add1 = parse_input(&r#"[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]"#);
    assert_eq!(reduce(SnailNumber::Pair(
                Box::new(test_add1[0].clone()),
                Box::new(test_add1[1].clone()))),
            SnailNumber::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").0);

    let test_add2 = parse_input(&r#"[1,1]
[2,2]
[3,3]
[4,4]"#);
    assert_eq!(part1(&test_add2), 445);

    let test_add3 = parse_input(&r#"[1,1]
[2,2]
[3,3]
[4,4]
[5,5]"#);
    assert_eq!(part1(&test_add3), 791);

    let test_add4 = parse_input(&r#"[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]"#);
    assert_eq!(part1(&test_add4), 1137);

    let test_add5 = parse_input(&r#"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"#);
    assert_eq!(part1(&test_add5), 3488);

    let tests = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 4140);
    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(18)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
