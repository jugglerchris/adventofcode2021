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
fn part2(data: &[Link]) -> usize {
    unimplemented!()
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
//    assert_eq!(part2(&data), 0);
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
