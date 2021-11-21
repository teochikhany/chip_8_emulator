use crate::ram::Ram;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu
{
    stack: [u16; 17],    // TODO: make this a array of 16 element not 17
    vx: [u8; 16],
    pub pc: u16,
    sp: usize,
    i: u16,
}

impl Cpu
{
    pub fn new() -> Cpu
    {
        Cpu 
        {
            stack: [0; 17],
            vx: [0; 16],
            pc: PROGRAM_START,
            sp: 0,
            i: 0,
        }
    }

    pub fn write_register(&mut self, register_index: u8, value:u8)
    {
        self.vx[register_index as usize] = value;
    }

    pub fn read_register(&self, register_index: u8) -> u8
    {
        return self.vx[register_index as usize];
    }

    #[allow(dead_code)] // TODO: remove this later if not used
    pub fn debug(&self)
    {
        println!("\nCpu Stats");
        println!("stack size: {:}", self.stack.len());
        println!("registers: {:?}", self.vx);
        println!("pc counter: {:}", self.pc);
        println!("i register: {:}", self.i);
    }

    pub fn run_instruction(&mut self, ram: &mut Ram)
    {
        let hi = ram.read_byte(self.pc) as u16;
        let lo = ram.read_byte(self.pc + 1) as u16;

        let instruction: u16 = (hi << 8) | lo;

        let nnn = instruction & 0x0FFF;
        let kk = (instruction & 0x0FF) as u8;
        let x = ((instruction & 0x0F00) >> 8 ) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;
        let n = (instruction & 0x00F) as u8;

        let t = ((instruction & 0xF000) >> 12 ) as u8;

        match t
        {
            0x0 => 
            {
                match kk
                {
                    0xE0 => println!("E0, clear display"),
                    0xEE => 
                    {
                        // return from subroutine
                        self.pc = self.stack[self.sp]; 
                        self.sp -= 1;
                        return;
                    },
                    _ => println!("unknown instruction in 0x0")
                }
            },

            0x1 => {self.pc = nnn; return}, // JP addr 
            0x2 => 
            {
                //  CALL addr
                self.sp += 1; 
                self.stack[self.sp] = self.pc + 2;
                self.pc = nnn;
                return;
            },

            0x3 => 
            {
                // SE Vx, byte
                if self.read_register(x) == kk
                {
                    self.pc += 2;
                }
            },

            0x4 => 
            {
                // SNE Vx, byte
                if self.read_register(x) != kk
                {
                    self.pc += 2;
                }
            },

            0x5 => 
            {
                // SE Vx, Vy
                if self.read_register(x) == self.read_register(y)
                {
                    self.pc += 2;
                }
            },
            0x6 => 
            {
                // LD Vx, byte
                self.write_register(x, kk);
            },

            0x7 => 
            {
                // ADD Vx, byte
                let c_vx = self.read_register(x);
                self.write_register(x, c_vx + kk);  // TODO: wrapping_add what is it and should i use it?
            },

            0x8 =>
            {
                let vx = self.read_register(x);
                let vy = self.read_register(y);

                match n
                {
                    0x0 => { self.write_register(x, vy) }, 
                    0x1 => { self.write_register(x, vx | vy) }, 
                    0x2 => { self.write_register(x, vx & vy) }, 
                    0x3 => { self.write_register(x, vx ^ vy) }, 
                    0x4 => { 
                            // ADD Vx, Vy
                            let total: u16 = vx as u16 + vy as u16;
                            self.write_register(x, total as u8);
                            self.write_register(0xF, if total > 255  {1} else {0} );
                        }, 

                    0x5 => { 
                            // SUB Vx, Vy 
                            // TODO: check if I should use signed int instead of unsigned int
                            let total = vx - vy;
                            self.write_register(x, total);
                            self.write_register(0xF, if vx > vy {1} else {0} );
                        }, 

                    0x6 => {
                            self.write_register(0xF, vx & 0x1);
                            self.write_register(x, vx >> 1);
                        }, 
                    
                    0x7 => { 
                            // SUB Vy, Vx 
                            // TODO: check if I should use signed int instead of unsigned int
                            let total = vy - vx;
                            self.write_register(x, total);
                            self.write_register(0xF, if vy > vx {1} else {0} );
                        }, 

                    0xE => {
                            self.write_register(0xF, (vx & 0x80) >> 7);
                            self.write_register(x, vx << 1);
                        },  

                    _ => println!("unknown instruction in 0x8")
                }
            },

            0x9 => println!("0x9"), 
            0xA => println!("0xA"), 
            0xB => println!("0xB"), 
            0xC => println!("0xC"), 
            0xD => println!("0xD"), 

            0xE =>
            {
                match kk
                {
                    0xA1 => println!("0x0xA1"), 
                    0x9E => println!("0x0x9E"), 
                    _ => println!("unknown instruction in 0xE")
                }
            },

            0xF => 
            {
                match kk
                {
                    0x07 => println!("0x07"), 
                    0x1A => println!("0x1A"), 
                    0x15 => println!("0x15"), 
                    0x18 => println!("0x18"), 
                    0x1E => println!("0x1E"), 
                    0x29 => println!("0x29"), 
                    0x33 => println!("0x33"), 
                    0x55 => println!("0x55"), 
                    0x65 => println!("0x65"), 
                    _ => println!("unknown instruction in 0xF")
                }
            }


            _ => println!("not implementaed yet")
        }

        self.pc += 2; // FIXME: this shouldn't always run, when self.pc is set in the instruction, this should be skipped
    }
}