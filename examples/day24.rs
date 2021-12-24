use adventofcode2021::{get_input,parse_lines,regex_parser};
use std::rc::Rc;

type Value = isize;

#[derive(Debug, Copy, Clone)]
pub enum Register {
    W = 0,
    X = 1,
    Y = 2,
    Z = 3,
}

impl std::str::FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Register::W),
            "x" => Ok(Register::X),
            "y" => Ok(Register::Y),
            "z" => Ok(Register::Z),
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Inp(Register),
    Add(Register, Register),
    AddI(Register, Value),
    Mul(Register, Register),
    MulI(Register, Value),
    Div(Register, Register),
    DivI(Register, Value),
    Mod(Register, Register),
    ModI(Register, Value),
    Eql(Register, Register),
    EqlI(Register, Value),
}

regex_parser!(parse_instruction: Instruction {
    INP = r#"^inp ([wxyz])$"# => |reg: Register| Instruction::Inp(reg),
    ADD = r#"^add ([wxyz]) ([wxyz])$"# => |dest: Register, src: Register| Instruction::Add(dest, src),
    ADDI = r#"^add ([wxyz]) (-?\d+)$"# => |dest: Register, src: Value| Instruction::AddI(dest, src),
    MUL = r#"^mul ([wxyz]) ([wxyz])$"# => |dest: Register, src: Register| Instruction::Mul(dest, src),
    MULI = r#"^mul ([wxyz]) (-?\d+)$"# => |dest: Register, src: Value| Instruction::MulI(dest, src),
    DIV = r#"^div ([wxyz]) ([wxyz])$"# => |dest: Register, src: Register| Instruction::Div(dest, src),
    DIVI = r#"^div ([wxyz]) (-?\d+)$"# => |dest: Register, src: Value| Instruction::DivI(dest, src),
    MOD = r#"^mod ([wxyz]) ([wxyz])$"# => |dest: Register, src: Register| Instruction::Mod(dest, src),
    MODI = r#"^mod ([wxyz]) (-?\d+)$"# => |dest: Register, src: Value| Instruction::ModI(dest, src),
    EQL = r#"^eql ([wxyz]) ([wxyz])$"# => |dest: Register, src: Register| Instruction::Eql(dest, src),
    EQLI = r#"^eql ([wxyz]) (-?\d+)$"# => |dest: Register, src: Value| Instruction::EqlI(dest, src)
});

type Data = Vec<Instruction>;
fn parse_input(input: &str) -> Data {
    parse_lines(input)
}

#[derive(Debug,Clone)]
enum AluResult {
    Blocked,  // Waited for input which didn't arrive
    Error,    // Div by zero or similar
    Success([Value; 4]),
}

struct Cpu<'a> {
    instructions: &'a [Instruction],
    pc: usize,
    regs: [Value; 4],
    inputs: Vec<Value>,
}

impl<'a> Cpu<'a> {
    pub fn new(instructions: &'a [Instruction]) -> Self {
        Cpu {
            instructions,
            pc: 0,
            regs: [0, 0, 0, 0],
            inputs: vec![],
        }
    }
    pub fn get(&self, reg: Register) -> Value {
        self.regs[reg as usize]
    }
    pub fn set(&mut self, reg: Register, value: Value) {
        self.regs[reg as usize] = value;
    }
    pub fn step(&mut self) -> AluResult {
        let result = match self.instructions[self.pc] {
            Instruction::Inp(reg) => match self.inputs.pop() {
                None => return AluResult::Blocked,
                Some(v) => {
                    self.set(reg, v);
                }
            }
            Instruction::Add(dst, src) => {
                self.set(dst, self.get(dst) + self.get(src));
            }
            Instruction::AddI(dst, v) => {
                self.set(dst, self.get(dst) + v);
            }
            Instruction::Mul(dst, src) => {
                self.set(dst, self.get(dst) * self.get(src));
            }
            Instruction::MulI(dst, v) => {
                self.set(dst, self.get(dst) * v);
            }
            Instruction::Div(dst, src) => {
                if self.get(src) == 0 {
                    return AluResult::Error;
                }
                self.set(dst, self.get(dst) / self.get(src));
            }
            Instruction::DivI(dst, v) => {
                if v == 0 {
                    return AluResult::Error;
                }
                self.set(dst, self.get(dst) / v);
            }
            Instruction::Mod(dst, src) => {
                let a = self.get(dst);
                let b = self.get(src);
                if a < 0 || b <= 0 {
                    return AluResult::Error;
                }
                self.set(dst, a % b);
            }
            Instruction::ModI(dst, b) => {
                let a = self.get(dst);
                if a < 0 || b <= 0 {
                    return AluResult::Error;
                }
                self.set(dst, a % b);
            }
            Instruction::Eql(dst, src) => {
                self.set(dst, if self.get(dst) == self.get(src) { 1 } else { 0 });
            }
            Instruction::EqlI(dst, v) => {
                self.set(dst, if self.get(dst) == v { 1 } else { 0 });
            }
        };
        self.pc += 1;
        AluResult::Success(self.regs)
    }

    pub fn run_with_input(&mut self, mut values: Vec<Value>) -> AluResult {
        values.reverse();
        self.inputs = values;
        self.pc = 0;
        self.regs = [0, 0, 0, 0];

        while self.pc < self.instructions.len() {
            match self.step() {
                res@AluResult::Blocked => return res,
                res@AluResult::Error => return res,
                AluResult::Success(_) => (),
            }
        }
        AluResult::Success(self.regs)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Expr {
    Lit(Value),
    Inp(u8),
    Add(Rc<Expr>, Rc<Expr>),
    Mul(Rc<Expr>, Rc<Expr>),
    Div(Rc<Expr>, Rc<Expr>),
    Mod(Rc<Expr>, Rc<Expr>),
    Eql(Rc<Expr>, Rc<Expr>),
}

fn symbolic(instructions: &[Instruction]) -> Expr {
    use Expr::*;
    let mut regs = [Lit(0), Lit(0), Lit(0), Lit(0)];
    let mut num_inputs = 0;
    for insn in instructions {
        match insn {
            Instruction::Inp(reg) => {
                regs[*reg as usize] = Inp(num_inputs);
                num_inputs += 1;
            }
            Instruction::Add(dst, src) => {
                let a = regs[*dst as usize].clone();
                let b = regs[*src as usize].clone();
                if let (Lit(aa), Lit(bb)) = (&a, &b) {
                    regs[*dst as usize] = Lit(aa+bb);
                } else {
                    regs[*dst as usize] = Add(Rc::new(a), Rc::new(b));
                }
            }
            Instruction::AddI(dst, src) => {
                let a = regs[*dst as usize].clone();
                let b = Lit(*src);
                if let (Lit(aa), Lit(bb)) = (&a, &b) {
                    regs[*dst as usize] = Lit(aa+bb);
                } else {
                    regs[*dst as usize] = Add(Rc::new(a), Rc::new(b));
                }
            }
            Instruction::Mul(dst, src) => {
                let a = regs[*dst as usize].clone();
                let b = regs[*src as usize].clone();
                if a == Lit(0) || b == Lit(0) {
                    regs[*dst as usize] = Lit(0);
                } else if let (Lit(aa), Lit(bb)) = (&a, &b) {
                    regs[*dst as usize] = Lit(aa*bb);
                } else if b == Lit(1) {
                    regs[*dst as usize] = a;
                } else {
                    regs[*dst as usize] = Mul(Rc::new(a), Rc::new(b));
                }
            }
            Instruction::MulI(dst, src) => {
                let a = regs[*dst as usize].clone();
                let b = Lit(*src);
                if a == Lit(0) || b == Lit(0) {
                    regs[*dst as usize] = Lit(0);
                } else if let (Lit(aa), Lit(bb)) = (&a, &b) {
                    regs[*dst as usize] = Lit(aa*bb);
                } else if b == Lit(1) {
                    regs[*dst as usize] = a;
                } else {
                    regs[*dst as usize] = Mul(Rc::new(a), Rc::new(b));
                }
            }
            Instruction::Div(dst, src) => {
                let a = regs[*dst as usize].clone();
                let b = regs[*src as usize].clone();
                if b == Lit(1) {
                    regs[*dst as usize] = a;
                } else if let (Lit(aa), Lit(bb)) = (&a, &b) {
                    regs[*dst as usize] = Lit(aa/bb);
                } else {
                    regs[*dst as usize] = Div(Rc::new(a), Rc::new(b));
                }
            }
            Instruction::DivI(dst, src) => {
                let a = regs[*dst as usize].clone();
                let b = Lit(*src);
                if b == Lit(1) {
                    regs[*dst as usize] = a;
                } else if let (Lit(aa), Lit(bb)) = (&a, &b) {
                    regs[*dst as usize] = Lit(aa/bb);
                } else {
                    regs[*dst as usize] = Div(Rc::new(a), Rc::new(b));
                }
            }
            Instruction::Mod(dst, src) => {
                let a = regs[*dst as usize].clone();
                let b = regs[*src as usize].clone();
                if a == Lit(0) {
                    regs[*dst as usize] = a;
                } else if let (Lit(aa), Lit(bb)) = (&a, &b) {
                    regs[*dst as usize] = Lit(aa%bb);
                } else {
                    regs[*dst as usize] = Mod(Rc::new(a), Rc::new(b));
                }
            }
            Instruction::ModI(dst, src) => {
                let a = regs[*dst as usize].clone();
                let b = Lit(*src);
                if a == Lit(0) {
                    regs[*dst as usize] = a;
                } else if let (Lit(aa), Lit(bb)) = (&a, &b) {
                    regs[*dst as usize] = Lit(aa%bb);
                } else {
                    regs[*dst as usize] = Mod(Rc::new(a), Rc::new(b));
                }
            }
            Instruction::Eql(dst, src) => {
                let a = regs[*dst as usize].clone();
                let b = regs[*src as usize].clone();
                if let (Lit(aa), Lit(bb)) = (&a, &b) {
                    regs[*dst as usize] = Lit(if aa == bb { 1 } else { 0 });
                } else {
                    regs[*dst as usize] = Eql(Rc::new(a), Rc::new(b));
                }
            }
            Instruction::EqlI(dst, src) => {
                let a = regs[*dst as usize].clone();
                let b = Lit(*src);
                if let (Lit(aa), Lit(bb)) = (&a, &b) {
                    regs[*dst as usize] = Lit(if aa == bb { 1 } else { 0 });
                } else {
                    regs[*dst as usize] = Eql(Rc::new(a), Rc::new(b));
                }
            }
        }
    }
    regs[3].clone()
}

fn part1(data: &Data) -> isize {
    let mut cpu = Cpu::new(data);
    let mut input = vec![9; 14];
    'outer: for place in (0..14).rev() {
        let mut best = isize::MAX;
        let mut best_digit = 0;
        for digit in 1..9 {
            input[place] = digit;
            let result = cpu.run_with_input(input.clone());
            match result {
                AluResult::Blocked => continue,
                AluResult::Error => continue,
                AluResult::Success([_, _, _, z]) => {
                    if z == 0 {
                        break 'outer;
                    }
                    if z < best {
                        best = z;
                        best_digit = digit;
                    }
                }
            }
        }
        input[place] = best_digit;
    }
    let mut n = 0;
    for &digit in &input {
        n = (n * 10) + digit;
    }
    n
}
fn part2(data: &Data) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let tests = r#""#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 0);
    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(24)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
