use std::convert::TryFrom;
pub type Int = isize;
use std::io::{Write};
use std::fmt::{Debug,Display};

#[derive(Debug)]
pub enum Error {
    UnknownInstruction,
    IndexOutOfBounds,
    NegativeIndex,
    InvalidMode,
    Unknown,
    InputNeeded,
}

impl std::error::Error for Error {
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        <Self as Debug>::fmt(self, f)
    }
}

#[derive(Debug)]
pub enum Event {
    Output(Int),
    InputNeeded,
    Halted,
}

pub struct IntcodeMachine {
    data: Vec<Int>,
    pc: usize,
    rel_base: Int,
    halted: bool,
    inputs: Vec<Int>,
    outputs: Vec<Int>,
}

impl IntcodeMachine {
    pub fn new(data: &[Int]) -> IntcodeMachine {
        IntcodeMachine {
            data: Vec::from(data),
            pc: 0,
            rel_base: 0,
            halted: false,
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn step(&mut self) -> Result<(), Error> {
        let insn = self.data[self.pc];
        let opcode = insn % 100;
        let (mode0, mode1, mode2) = {
            let mut val = insn / 100;
            let mode0 = val % 10;
            val /= 10;
            let mode1 = val % 10;
            val /= 10;
            let mode2 = val % 10;

            (mode0, mode1, mode2)
        };
        // Speculatively get arguments
        let arg0 = self.get_u(self.pc + 1)?;
        let val0 = self.get_arg(arg0, mode0)?;
        let arg1 = self.get_u(self.pc + 2)?;
        let val1 = self.get_arg(arg1, mode1)?;
        match opcode {
            1 => {
                let addr = self.get_arg_addr(self.get_u(self.pc + 3)?, mode2);
                self.set(addr, val0.checked_add(val1).unwrap());
                self.pc += 4;
            }
            2 => {
                let addr = self.get_arg_addr(self.get_u(self.pc + 3)?, mode2);
                self.set(addr, val0.checked_mul(val1).unwrap());
                self.pc += 4;
            }
            3 => {
                let val = self.get_input()?;
                let addr = self.get_arg_addr(self.get_u(self.pc + 1)?, mode0);
                self.set(addr, val);
                self.pc += 2;
            }
            4 => {
                self.output(val0);
                self.pc += 2;
            }
            5 => {  // Jump if true
                if val0 != 0 {
                    self.pc = usize::try_from(val1).unwrap();
                } else {
                    self.pc += 3;
                }
            }
            6 => {  // Jump if false
                if val0 == 0 {
                    self.pc = usize::try_from(val1).unwrap();
                } else {
                    self.pc += 3;
                }
            }
            7 => { // less than
                let addr = self.get_arg_addr(self.get_u(self.pc + 3)?, mode2);
                self.set(addr, if val0 < val1 { 1 } else { 0 });
                self.pc += 4;
            }
            8 => { // equals
                let addr = self.get_arg_addr(self.get_u(self.pc + 3)?, mode2);
                self.set(addr, if val0 == val1 { 1 } else { 0 });
                self.pc += 4;
            }
            9 => { // Adjust relative base
                self.rel_base += val0;
                self.pc += 2;
            }
            99 => {
                self.halted = true;
            }
            _ => {
                return Err(Error::UnknownInstruction);
                //panic!("Unknown instruction {} at {}", insn, self.pc);
            }
        }
        Ok(())
    }

    pub fn run_until_halt(&mut self) -> Result<(), Error> {
        while !self.halted {
            self.step()?;
        }
        Ok(())
    }

    pub fn run_until_output(&mut self) -> Result<Option<Int>, Error> {
        while !self.halted {
            if self.outputs.len() > 0 {
                return Ok(Some(self.outputs.remove(0)));
            }
            self.step()?;
        }
        Ok(None)
    }

    pub fn run_until_event(&mut self) -> Result<Event, Error> {
        while !self.halted {
            if self.outputs.len() > 0 {
                return Ok(Event::Output(self.outputs.remove(0)));
            }
            match self.step() {
                Ok(_) => {}
                Err(Error::InputNeeded) => {
                    return Ok(Event::InputNeeded);
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        Ok(Event::Halted)
    }

    pub fn run_ascii(&mut self, input: &str) -> Result<String, Error> {
        self.send_input_ascii(input);
        while !self.halted {
            match self.step() {
                Ok(_) => {}
                Err(Error::InputNeeded) => {
                    break;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        let result = self.outputs.drain(..).map(|v| v as u8 as char).collect();
        Ok(result)
    }

    pub fn run_ascii_interactive(&mut self) -> Result<Event, Error> {
        loop {
            match self.run_until_event()? {
                Event::Output(c) => {
                    print!("{}", c as u8 as char);
                    std::io::stdout().flush().unwrap();
                }
                Event::InputNeeded => {
                    let mut s = String::new();
                    let _count = std::io::stdin().read_line(&mut s);
                    self.send_input_ascii(&s);
                }
                Event::Halted => { break; }
            }
        }
        Ok(Event::Halted)
    }

    pub fn run_ascii_fixed_input(&mut self, s: &str) -> Result<Vec<Int>, Error> {
        self.send_input_ascii(s);
        let mut result = Vec::new();
        loop {
            match self.run_until_event()? {
                Event::Output(c) => {
                    result.push(c);
                }
                Event::InputNeeded => {
                    panic!();
                }
                Event::Halted => { break; }
            }
        }
        Ok(result)
    }

    pub fn get(&self, idx: Int) -> Result<Int, Error> {
        let idx = usize::try_from(idx).map_err(|_| Error::NegativeIndex)?;
        self.get_u(idx)
    }

    pub fn get_u(&self, idx: usize) -> Result<Int, Error> {
        if idx >= self.data.len() {
            return Ok(0);
        }
        self.data.get(idx).cloned().ok_or(Error::IndexOutOfBounds)
    }

    fn get_arg(&self, idx: Int, mode: Int) -> Result<Int, Error> {
        match mode {
            0 => self.get(idx),
            1 => Ok(idx),  // immediate
            2 => self.get(idx + self.rel_base),
            _ => Err(Error::InvalidMode),
        }
    }

    fn get_arg_addr(&self, idx: Int, mode: Int) -> Int {
        match mode {
            0 => idx,
            2 => idx + self.rel_base,
            _ => { panic!("Invalid mode for output argument {}", mode); },
        }
    }

    fn get_input(&mut self) -> Result<Int, Error> {
        if self.inputs.len() > 0 {
            Ok(self.inputs.remove(0))
        } else {
            Err(Error::InputNeeded)
        }
    }

    fn output(&mut self, val: Int) {
        self.outputs.push(val);
    }

    pub fn set(&mut self, idx: Int, val: Int) {
        let idx = usize::try_from(idx).unwrap();
        if idx >= self.data.len() {
            self.data.resize(idx+1, 0);
        }
        self.data[idx] = val;
    }

    pub fn get_data(&self, start: usize, len: usize) -> &[Int] {
        &self.data[start..start+len]
    }

    pub fn send_input(&mut self, val: Int) {
        self.inputs.push(val);
    }

    pub fn send_input_ascii(&mut self, input: &str) {
        for c in input.chars() {
            self.send_input(c as Int);
        }
    }

    pub fn get_outputs(&self) -> &[Int] {
        &self.outputs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_state(memin: &[Int], memout: &[Int]) {
        let mut machine = IntcodeMachine::new(memin);

        machine.run_until_halt();
        assert_eq!(machine.get_data(0, memout.len()), memout);
    }

    fn assert_state_inout(memin: &[Int], input: &[Int], memout: &[Int], output: &[Int]) {
        let mut machine = IntcodeMachine::new(memin);
        for val in input {
            machine.send_input(*val);
        }
        machine.run_until_halt();
        assert_eq!(machine.get_data(0, memout.len()), memout);
        assert_eq!(machine.get_outputs(), output);
    }

    fn assert_inout(memin: &[Int], input: &[Int], output: &[Int]) {
        let mut machine = IntcodeMachine::new(memin);
        for val in input {
            machine.send_input(*val);
        }
        machine.run_until_halt().unwrap();
        assert_eq!(machine.get_outputs(), output);
    }

    #[test]
    fn test_machine() {
        assert_state(&[1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]);
        assert_state(&[2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]);
        assert_state(&[2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]);
        assert_state(&[1, 1, 1, 4, 99, 5, 6, 0, 99] ,&[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_inout() {
        assert_state_inout(&[3,0,4,0,99], &[77], &[77,0,4,0,99], &[77]);
    }

    #[test]
    fn test_conditionals() {
        assert_state_inout(&[3,9,8,9,10,9,4,9,99,-1,8], &[8], &[3,9,8,9,10,9,4,9,99,1,8], &[1]);
        assert_state_inout(&[3,9,8,9,10,9,4,9,99,-1,8], &[9], &[3,9,8,9,10,9,4,9,99,0,8], &[0]);
    }

    #[test]
    fn test_rel_base() {
        assert_inout(&[109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99],
                    &[], &[109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
        assert_inout(&[1102,34915192,34915192,7,4,7,99,0], &[], &[1219070632396864]);
        assert_inout(&[104,1125899906842624,99], &[], &[1125899906842624]);
    }
}

pub fn run_with_input(data: &[Int], noun: Int, verb: Int) -> Result<Int, Error> {
    let mut machine = IntcodeMachine::new(data);
    machine.set(1, noun);
    machine.set(2, verb);
    machine.run_until_halt()?;
    machine.get(0)
}

