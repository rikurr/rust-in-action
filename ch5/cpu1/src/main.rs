struct CPU {
    current_operation: u16,
    register: [u8; 2],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    fn run(&mut self) {
        // loop {
        let opcode = self.read_opcode();
        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = (opcode & 0x000F) as u8;

        match (c, x, y, d) {
            (_x08, _, _, _x04) => self.add_xy(x, y),
            _ => todo!("opcode {:04x}", opcode),
        }
        // }

        println!("{:b}", opcode);
        println!("{}", c);
        println!("{}", x);
        println!("{}", y);
        println!("{:b}", d);
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.register[x as usize] += self.register[y as usize];
    }
}

fn main() {
    let mut cpu = CPU {
        current_operation: 0,
        register: [0; 2],
    };

    cpu.current_operation = 0x8014;
    cpu.register[0] = 5;
    cpu.register[1] = 10;

    cpu.run();

    assert_eq!(cpu.register[0], 15);
    println!("5 + 10 = {}", cpu.register[0]);
}
