#[path = "ram.rs"] mod ram;

use ram::Ram;

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

    pub fn run_instruction(&mut self)
    {
        
    }
}