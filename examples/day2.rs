use adventofcode2021::{get_input,parse_lines, regex_parser};

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
}

regex_parser!(parse_command: Command {
    FWD = r#"^forward (\d+)$"# => |inc: isize| Command::Forward(inc),
    DOWN = r#"^down (\d+)$"# => |inc: isize| Command::Down(inc),
    UP = r#"^up (\d+)$"# => |inc: isize| Command::Up(inc)
});

fn main() -> std::io::Result<()>{
    let input = get_input(2)?;

    let commands: Vec<Command> = parse_lines(&input);

    use Command::*;
    // Part 1
    {
        let mut x = 0;
        let mut y = 0;
        for command in &commands {
            match command {
                Forward(n) => {
                    x += n;
                }
                Up(n) => {
                    y -= n;
                }
                Down(n) => {
                    y += n;
                }
            }
        }
        println!("{}", x*y);
    }

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
