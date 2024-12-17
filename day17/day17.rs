use std::io;

#[derive(Debug)]
#[allow(non_snake_case)]
struct CPU {
    A: u32,
    B: u32,
    C: u32,
    pc: usize,
}

impl CPU {
    fn combo_operand(&self, operand: u32) -> u32 {
        match operand {
            0..=3 => operand,
            4 => self.A,
            5 => self.B,
            6 => self.C,
            _ => panic!("invalid combo operand: {}", operand),
        }
    }

    fn execute(&mut self, opcode: u32, operand: u32, output: &mut Vec<u32>) {
        self.pc += 2;
        match opcode {
            0 => self.A >>= self.combo_operand(operand),
            1 => self.B ^= operand,
            2 => self.B = self.combo_operand(operand) & 0x7,
            3 => {
                if self.A != 0 {
                    self.pc = operand as usize;
                }
            }
            4 => self.B ^= self.C,
            5 => output.push(self.combo_operand(operand) & 0x7),
            6 => self.B = self.A >> self.combo_operand(operand),
            7 => self.C = self.A >> self.combo_operand(operand),
            _ => panic!("invalid opcode: {}", opcode),
        }
    }

    fn run(&mut self, program: &[u32], output: &mut Vec<u32>) {
        while self.pc + 1 < program.len() {
            self.execute(program[self.pc], program[self.pc + 1], output);
        }
    }
}

fn main() {
    let input = io::read_to_string(io::stdin()).expect("error reading input");
    let (registers, program) = input.split_once("\n\n").expect("error parsing input");
    let registers: Vec<_> = registers
        .lines()
        .map(|register| {
            let (_, value) = register.split_once(": ").expect("error parsing register");
            value.parse::<u32>().expect("error parsing register value")
        })
        .collect();
    let (_, program) = program.split_once(": ").expect("error parsing program");
    let program: Vec<_> = program
        .trim()
        .split(',')
        .map(|instruction| {
            instruction
                .parse::<u32>()
                .expect("error parsing instruction")
        })
        .collect();

    assert!(registers.len() == 3);
    let mut cpu = CPU {
        A: registers[0],
        B: registers[1],
        C: registers[2],
        pc: 0,
    };

    let mut output = Vec::new();
    cpu.run(&program, &mut output);
    println!(
        "part 1: {}",
        output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}
