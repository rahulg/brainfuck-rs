use std::fs::File;
use std::io;
use std::io::{Read, BufRead, BufReader};
use std::env;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Right,
    Left,
    Increment,
    Decrement,
    Output,
    Input,
    JumpOpen(usize),
    JumpClose(usize),
    Comment(char),
    End,
}

impl From<char> for Instruction {
    fn from(c: char) -> Instruction {
        match c {
            '>' => Instruction::Right,
            '<' => Instruction::Left,
            '+' => Instruction::Increment,
            '-' => Instruction::Decrement,
            '.' => Instruction::Output,
            ',' => Instruction::Input,
            '[' => Instruction::JumpOpen(0),
            ']' => Instruction::JumpClose(0),
            c => Instruction::Comment(c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Programme {
    instructions: Vec<Instruction>,
    index: usize,
}

impl Programme {

    fn parse<R>(r: R) -> Result<Programme, io::Error>
        where R: BufRead
    {

        let mut instructions = Vec::new();
        let mut sq_brackets = Vec::new();

        let mut instr_id = 0;
        for b in r.bytes() {

            let c = try!(b);

            let mut instr = Instruction::from(c as char);

            match instr {
                Instruction::Comment(_) => continue,
                Instruction::JumpOpen(_) => sq_brackets.push(instr_id),
                Instruction::JumpClose(_) => {
                    let i_open = sq_brackets.pop().unwrap();
                    instructions[i_open] = Instruction::JumpOpen(instr_id - i_open);
                    instr = Instruction::JumpClose(instr_id - i_open);
                },
                _ => {},
            }
            instructions.push(instr);
            instr_id += 1;

        }

        instructions.push(Instruction::End);

        Ok(Programme{instructions: instructions, index: 0})

    }

    fn step(&mut self, ms: &mut MachineState) {
        match self.instructions[self.index] {
            Instruction::Right => {
                ms.index += 1;
                let len = ms.tape.len();
                if ms.index >= len {
                    ms.tape.extend(vec![0u8; len]);
                }
            },
            Instruction::Left => ms.index -= 1,
            Instruction::Increment => ms.tape[ms.index] = ms.tape[ms.index].wrapping_add(1),
            Instruction::Decrement => ms.tape[ms.index] = ms.tape[ms.index].wrapping_sub(1),
            Instruction::Output => print!("{}", ms.tape[ms.index] as char),
            Instruction::Input => ms.tape[ms.index] = match io::stdin().bytes().next() {
                Some(r) => r.unwrap_or(0),
                None => 0,
            },
            Instruction::JumpOpen(offset) => if ms.tape[ms.index] == 0 {
                self.index += offset;
            },
            Instruction::JumpClose(offset) => if ms.tape[ms.index] != 0 {
                self.index -= offset + 1;
            },
            Instruction::Comment(_) => {},
            Instruction::End => return,
        }
        self.index += 1;
    }

    fn execute(&mut self, mut ms: &mut MachineState) {
        while self.instructions[self.index] != Instruction::End {
            self.step(&mut ms);
        }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct MachineState {
    tape: Vec<u8>,
    index: usize,
}

fn main() {

    let argv: Vec<String> = env::args().collect();

    let file = File::open(&argv[1]).unwrap();
    let mut buffer = BufReader::new(file);

    let mut prgm = Programme::parse(&mut buffer).unwrap();

    let mut state = MachineState{
        tape: vec![0u8; 8192],
        .. MachineState::default()
    };

    prgm.execute(&mut state);

}
