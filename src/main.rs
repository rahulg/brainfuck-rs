use std::fs::File;
use std::io;
use std::io::Read;
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

    fn parse<S>(source: S) -> Programme
        where S: Into<String>
    {

        let s = source.into();

        let mut instructions = Vec::new();
        let mut braces = Vec::new();

        let mut instr_id = 0;
        for c in s.chars() {

            let mut instr = Instruction::from(c);

            match instr {
                Instruction::Comment(_) => continue,
                Instruction::JumpOpen(_) => braces.push(instr_id),
                Instruction::JumpClose(_) => {
                    let i_open = braces.pop().unwrap();
                    instructions[i_open] = Instruction::JumpOpen(instr_id - i_open);
                    instr = Instruction::JumpClose(instr_id - i_open);
                },
                _ => {},
            }
            instructions.push(instr);
            instr_id += 1;

        }

        instructions.push(Instruction::End);

        Programme{instructions: instructions, index: 0}

    }

    fn step(&mut self, ms: &mut MachineState) {
        match self.instructions[self.index] {
            Instruction::Right => ms.index += 1,
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

    let mut file = File::open(&argv[1]).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let mut prgm = Programme::parse(buffer);

    let mut state = MachineState{
        tape: vec![0u8; 256],
        .. MachineState::default()
    };

    prgm.execute(&mut state);

}
