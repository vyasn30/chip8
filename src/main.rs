struct CPU {
    registers: [u8; 16],
    program_counter: usize,
    memory: [u8; 0x1000],
}
impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.program_counter;
        let opcode_byte0 = self.memory[p] as u16;
        let opcode_byte1: u16 = self.memory[p + 1] as u16;

        return opcode_byte0 << 8 | opcode_byte1;
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();

            self.program_counter += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d: u8 = ((opcode & 0x000F) >> 0) as u8;

            // println!("{}", opcode);

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0x8, _, _, 0x4) => self.add(x, y),
                _ => todo!("baaki chhe {:04x}", opcode),
            }
        }
    }

    fn add(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0
        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        program_counter: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;

    mem[0] = 0x80;
    mem[1] = 0x14;
    mem[2] = 0x80;
    mem[3] = 0x24;
    mem[4] = 0x80;
    mem[5] = 0x34;
    cpu.run();

    assert_eq!(cpu.registers[0], 35);

    println!("{}", cpu.registers[0]);
}
