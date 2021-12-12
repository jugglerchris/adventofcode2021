use adventofcode2021::{get_input,parse_lines, regex_parser};
use std::{collections::{HashMap, HashSet}, usize};

#[derive(Clone, Debug)]
pub struct Cave {
    name: String,
}

#[derive(Clone, Debug)]
pub struct Link(String, String);

regex_parser!(parse_link: Link {
    LINK = r#"^(\w+)-(\w+)$"# => |a: String, b: String| Link(a, b)
});

enum Action {
    Continue,
    Finish,
}

fn is_small(s: &str) -> bool {
    s.chars().next().unwrap().is_ascii_lowercase()
}

fn search<'st, 'a>(links: &HashMap<&'st str, Vec<&'st str>>,
          seen: &mut HashSet<&'st str>,
          start: &'st str,
          length: usize) ->usize
{
    let mut count = 0;
    #[cfg(test)]
    println!("{}{}", " ".repeat(length), start);
    if let Some(routes) = links.get(start) {
        if is_small(start) {
            seen.insert(start);
        }
        for dest in routes {
            let small = is_small(dest);
            if *dest == "end" {
                count += 1;
            } else if !small || !seen.contains(dest) {
                count += search(links, seen, dest, length+1);
            }
        }
        seen.remove(start);
    }
    count
}

fn part1(data: &[Link]) -> usize {
    let mut links: HashMap<&str, Vec<&str>> = HashMap::new();
    for Link(a, b) in data {
        links.entry(a).or_default().push(b);
        links.entry(b).or_default().push(a);
    }
    let mut seen = HashSet::new();
    search(&links, &mut seen, "start", 0)
}

fn search2<'st, 'a>(links: &HashMap<&'st str, Vec<&'st str>>,
          seen: &mut HashSet<&'st str>,
          dup: &mut Option<&'st str>,
          start: &'st str,
          length: usize) ->usize
{
    let mut count = 0;
    #[cfg(test)]
    println!("{}{}", " ".repeat(length), start);
    if let Some(routes) = links.get(start) {
        let was_dup = if is_small(start) {
            if seen.contains(start) {
                assert!(dup.is_none());
                *dup = Some(start);
                true
            } else {
                seen.insert(start);
                false
            }
        } else {
            false
        };
        for dest in routes {
            let small = is_small(dest);
            if *dest == "end" {
                count += 1;
                #[cfg(test)]
                println!("{} end", " ".repeat(length));
            } else if *dest == "start" {
                // Do nothing
            } else if !small || !seen.contains(dest) || dup.is_none() {
                count += search2(links, seen, dup, dest, length+1);
            }
        }
        if was_dup {
            *dup = None;
            assert!(seen.contains(start));
        } else {
            seen.remove(start);
        }
    }
    count
}

fn part2(data: &[Link]) -> usize {
    let mut links: HashMap<&str, Vec<&str>> = HashMap::new();
    for Link(a, b) in data {
        links.entry(a).or_default().push(b);
        links.entry(b).or_default().push(a);
    }
    let mut seen = HashSet::new();
    let mut dup = None;
    search2(&links, &mut seen, &mut dup, "start", 0)
}

#[test]
fn test() {
    let test1 = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end
"#;
    let test2 = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"#;
    let test3 = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
"#;
    let data1: Vec<Link> = parse_lines(&test1);
    let data2: Vec<Link> = parse_lines(&test2);
    let data3: Vec<Link> = parse_lines(&test3);

    assert_eq!(part1(&data1), 10);
    assert_eq!(part1(&data2), 19);
    assert_eq!(part1(&data3), 226);

    assert_eq!(part2(&data1), 36);
    assert_eq!(part2(&data2), 103);
    assert_eq!(part2(&data3), 3509);
}

fn main() -> std::io::Result<()>{
    let input = get_input(12)?;

    let data: Vec<Link> = parse_lines(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
