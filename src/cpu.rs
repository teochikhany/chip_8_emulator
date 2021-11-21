use crate::ram::Ram;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu
{
    stack: Vec<u8>,
    vx: [u8; 16],
    pc: u16,
    i: u16,
}

impl Cpu
{
    pub fn new() -> Cpu
    {
        Cpu 
        {
            stack: Vec::new(),
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
        }
    }

    pub fn write_register(&mut self, register_index: usize, value:u8)
    {
        self.vx[register_index] = value;
    }

    pub fn read_register(&mut self, register_index: usize) -> u8
    {
        return self.vx[register_index];
    }

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
                    0xEE => println!("EE, return subroutine"),
                    _ => println!("unknown instruction in 0x0")
                }
            },

            0x1 => println!("0x1, jump"),
            0x2 => println!("0x2, call subroutine"),
            0x3 => println!("0x3, skip instruction if =="),
            0x4 => println!("0x4, skip instruction if !="),
            0x5 => println!("0x5, skip instruction 2"),
            0x6 => println!("0x6, set register"),
            0x7 => println!("0x7, add"),

            0x8 =>
            {
                match n
                {
                    0x0 => println!("0x8xy0"), 
                    0x1 => println!("0x8xy1"), 
                    0x2 => println!("0x8xy2"), 
                    0x3 => println!("0x8xy3"), 
                    0x4 => println!("0x8xy4"), 
                    0x5 => println!("0x8xy5"), 
                    0x6 => println!("0x8xy6"), 
                    0x7 => println!("0x8xy7"), 
                    0xE => println!("0x8xyE"), 
                    _ => println!("unknown instruction in 0x8")
                }
            },

            0x9 => println!("0x9"), 
            0xA => println!("0xA"), 
            0xB => println!("0xB"), 
            0xD => println!("0xC"), 

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
    }
}