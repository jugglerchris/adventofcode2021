use adventofcode2021::{get_input,parse_lines,regex_parser};
use std::collections::{HashMap, HashSet};

#[derive(Clone,Debug)]
pub struct Entry {
    patterns: [String; 10],
    outputs: [String; 4],
}

// How many times each segment is shown in the digits
// 0..=9
static SEG_COUNTS: [usize; 7] = [
    8, // a
    6, // b
    8, // c
    7, // d
    4, // e
    9, // f
    7, // g
];

static DIGITS: [&[u8]; 10] = [
    b"abcefg",  // 0
    b"cf",      // 1
    b"acdeg",   // 2
    b"acdfg",   // 3
    b"bcdf",    // 4
    b"abdfg",   // 5
    b"abdefg",  // 6
    b"acf",     // 7
    b"abcdefg", // 8
    b"abcdfg",  // 9
];

const SEGMENTS: [u8;7] = [b'a', b'b', b'c', b'd', b'e', b'f', b'g'];

impl Entry {
    pub fn calc_output(&self) -> usize {
        // Map from puzzle letters to possible real segments
        let mut letter_to_segments: HashMap<u8, HashSet<u8>> = HashMap::new();
        let letter_counts: [usize;7] = b"abcdefg".iter()
            .map(move |l| {
                let mut count = 0;
                for p in &self.patterns {
                    for letter in p.as_bytes() {
                        if letter == l {
                            count += 1;
                        }
                    }
                }
                count
            })
            .collect::<Vec<usize>>().try_into().unwrap();
        for letter_idx in 0..7 {
            let mut hs = HashSet::new();
            let count = letter_counts[letter_idx];
            let c = b'a' + letter_idx as u8;
            for (j, cnt) in SEG_COUNTS.iter().enumerate() {
                let actual_letter = b'a' + j as u8;
                if count == *cnt {
                    hs.insert(actual_letter);
                }
            }
            letter_to_segments.insert(c, hs);
        }
        // Initial information
        for p in &self.patterns {
            if p.len() == 2 {
                // Must be a 1, so segments cf
                for c in p.as_bytes() {
                    let allowed: HashSet<u8> = (DIGITS[1]).iter().cloned().collect();
                    let newval = letter_to_segments[c].intersection(&allowed).cloned().collect();
                    *letter_to_segments.get_mut(c).unwrap() = newval;
                }
            } else if p.len() == 4 {
                for c in p.as_bytes() {
                    let allowed: HashSet<u8> = (DIGITS[4]).iter().cloned().collect();
                    let newval = letter_to_segments[c].intersection(&allowed).cloned().collect();
                    *letter_to_segments.get_mut(c).unwrap() = newval;
                }
            } else if p.len() == 3 {
                for c in p.as_bytes() {
                    let allowed: HashSet<u8> = (DIGITS[7]).iter().cloned().collect();
                    let newval = letter_to_segments[c].intersection(&allowed).cloned().collect();
                    *letter_to_segments.get_mut(c).unwrap() = newval;
                }
            } else if p.len() == 7 {
                // Must be an 8
            } else if p.len() == 6 {
                for c in p.as_bytes() {
                    let mut allowed: HashSet<u8> = DIGITS[0].iter().cloned().collect();
                    allowed = allowed.union(&DIGITS[6].iter().cloned().collect::<HashSet<_>>()).cloned().collect();
                    allowed = allowed.union(&DIGITS[9].iter().cloned().collect::<HashSet<_>>()).cloned().collect();
                    let newval = letter_to_segments[c].intersection(&allowed).cloned().collect();
                    *letter_to_segments.get_mut(c).unwrap() = newval;
                }
            } else if p.len() == 5 {
                for c in p.as_bytes() {
                    let mut allowed: HashSet<u8> = DIGITS[2].iter().cloned().collect();
                    allowed = allowed.union(&DIGITS[3].iter().cloned().collect::<HashSet<_>>()).cloned().collect();
                    allowed = allowed.union(&DIGITS[5].iter().cloned().collect::<HashSet<_>>()).cloned().collect();
                    let newval = letter_to_segments[c].intersection(&allowed).cloned().collect();
                    *letter_to_segments.get_mut(c).unwrap() = newval;
                }
            }
        }
        let mut mapping: HashMap<u8, u8> = HashMap::new();
        let mut used: HashSet<u8> = HashSet::new();
        while mapping.len() < 7 {
            for (c, hs) in &letter_to_segments {
                if mapping.contains_key(c) {
                    continue;
                }
                if hs.len() == 1 {
                    let target = hs.iter().next().unwrap();
                    mapping.insert(*c, *target);
                    used.insert(*target);
                }
            }
            for (_c, hs) in &mut letter_to_segments {
                if hs.len() > 1 {
                    for c in &used {
                        hs.remove(c);
                    }
                }
            }
        }
        let segs_to_digit: HashMap<String, usize> =
            DIGITS.iter()
                .enumerate()
                .map(|(i, s)| (String::from_utf8_lossy(*s).to_string(), i))
                .collect();
                  
        // We have a segment mapping
        let convert = |s: &String| {
            let mut chars = s.as_bytes()
             .iter()
             .map(|c| *mapping.get(c).unwrap() as char)
             .collect::<Vec<char>>();
            chars.sort();
            chars.into_iter().collect::<String>()
        };
        let mut v = 0;
        for o in &self.outputs {
            v = (v * 10) + segs_to_digit.get(&convert(o)).unwrap();
        }
        return v;
    }
}

regex_parser!(parse_entry: Entry {
    ENTRY = r#"^(\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) \| (\w+) (\w+) (\w+) (\w+)$"# => | p1: String,
                         p2: String,
                         p3: String,
                         p4: String,
                         p5: String,
                         p6: String,
                         p7: String,
                         p8: String,
                         p9: String,
                         p10: String,
                         o1: String,
                         o2: String,
                         o3: String,
                         o4: String| {
                             Entry {
                                 patterns: [p1, p2, p3, p4, p5, p6, p7, p8, p9, p10],
                                 outputs: [o1, o2, o3, o4],
                             }
                         }
});

fn part1(entries: &[Entry]) -> usize {
    let mut count = 0;
    for entry in entries {
        for output in &entry.outputs {
            if output.len() == 2  // 1
                || output.len() == 4 // 4
                    || output.len() == 3 // 7
                    || output.len() == 7 // 8
            {
                count += 1;
            }
        }
    }
    count
}

fn part2(entries: &[Entry]) -> usize {
    entries.iter()
           .map(|e| e.calc_output())
           .sum()
}

#[test]
fn test() {
    let input =
r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#;
    let entries : Vec<Entry> = parse_lines(input);

    assert_eq!(part1(&entries), 26);
    assert_eq!(part2(&entries), 61229);
}

fn main() -> std::io::Result<()>{
    let input = get_input(8)?;

    let entries: Vec<Entry> = parse_lines(&input);

    // Part 1
    println!("Part1: {}", part1(&entries));

    // Part 2
    println!("Part2: {}", part2(&entries));

    Ok(())
}
