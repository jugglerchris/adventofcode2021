use adventofcode2021::{get_input,parse_lines,regex_parser};

#[derive(Clone,Debug)]
pub struct Entry {
    patterns: [String; 10],
    outputs: [String; 4],
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
}

fn main() -> std::io::Result<()>{
    let input = get_input(8)?;

    let entries: Vec<Entry> = parse_lines(&input);

    // Part 1
    println!("Part1: {}", part1(&entries));

    // Part 2
    /*
    println!("{}", increments.iter()
                             .map(|mass| {
                                 let mut mass = *mass;
                                 let mut fuel = 0;
                                 while mass > 5 {
                                     let f = mass/3 - 2;
                                     fuel += f;
                                     mass = f;
                                 }
                                 fuel
                              })
                             .sum::<u32>());
    */

    Ok(())
}
