extern crate rand;

use crate::ram::Ram;
use crate::display::Display;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu
{
    stack: [u16; 16],
    vx: [u8; 16],
    pc: u16,
    sp: usize,
    i: u16,
    st: u8,
    dt: u8,
}

impl Cpu
{
    pub fn new() -> Cpu
    {
        Cpu 
        {
            stack: [0; 16],
            vx: [0; 16],
            pc: PROGRAM_START,
            sp: 0,
            i: 0,
            st: 0,
            dt: 0,
        }
    }


    fn write_register(&mut self, register_index: u8, value:u8)
    {
        self.vx[register_index as usize] = value;
    }

    fn read_register(&self, register_index: u8) -> u8
    {
        return self.vx[register_index as usize];
    }

    pub fn substract_dt(&mut self)
    {
        self.dt = self.dt.saturating_sub(1);
    }

    pub fn run_instruction(&mut self, ram: &mut Ram, display: &mut Display)
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
                    0xE0 => { display.clear_display() },
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
                self.write_register(x, c_vx.wrapping_add(kk));
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
                            let total = vx.wrapping_sub(vy);
                            self.write_register(x, total);
                            self.write_register(0xF, if vx > vy {1} else {0} );
                        }, 

                    0x6 => {
                            self.write_register(0xF, vx & 0x1);
                            self.write_register(x, vx >> 1);
                        }, 
                    
                    0x7 => { 
                            // SUB Vy, Vx 
                            let total = vy.wrapping_sub(vx);
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

            0x9 => 
            {
                // SNE Vx, Vy
                let vx = self.read_register(x);
                let vy = self.read_register(y);

                if vx != vy
                {
                    self.pc += 2;
                }
            }, 

            0xA => { self.i = nnn}, 
            0xB => { self.pc = nnn + self.read_register(0) as u16; return}, 
            0xC => { let random :u8 = rand::random(); self.write_register(x, random & kk) }, 

            0xD =>  
            {
                let sprite = ram.read(self.i, n as u16);
                let cood_x = self.read_register(x) as u16;
                let cood_y = self.read_register(y) as u16;

                let mut row = 0;
                for byte in sprite
                {
                    let mut column = 0;
                    let byte_str = format!("{:0>8b}", byte);

                    for bite in byte_str.chars()
                    {
                        let result = apply_scalling(display, cood_x, cood_y, row, column, bite);
                        self.write_register(0xf, result as u8);
        
                        column += 1;
                    }

                    row += 1;
                }

                display.update_pixel();
            },

            0xE =>
            {
                match kk
                {
                    0xA1 => { self.pc += 2; return; },  // FIXME: not implemented
                    0x9E => { return; },                // FIXME: not implemented
                    _ => println!("unknown instruction in 0xE")
                }
            },

            0xF => 
            {
                match kk
                {
                    0x07 => { self.write_register(x, self.dt) }, 
                    0x1A => println!("0x1A, waiting key press"),    // FIXME: not implemented
                    0x15 => { self.dt = self.read_register(x) }, 
                    0x18 => { self.st = self.read_register(x) }, 
                    0x1E => { self.i += self.read_register(x) as u16 }, 
                    0x29 => { self.i = self.read_register(x) as u16 * 5},

                    0x33 => {
                            // TODO: recheck this calculation
                            let vx = self.read_register(x);
                            ram.write_byte(self.i, vx / 100);
                            ram.write_byte(self.i + 1, (vx % 100) / 10);
                            ram.write_byte(self.i + 2, vx % 10);
                        }, 
                        
                    0x55 => {
                        for j in 0 ..= x
                        {
                            ram.write_byte(self.i + j as u16, self.read_register(j));
                        }

                        // self.i += x as u16 + 1;
                    }, 

                    0x65 => {
                        for j in 0 ..= x
                        {
                            let value = ram.read_byte(self.i + j as u16);
                            self.write_register(j, value);
                        }
                        
                        // self.i += x as u16 + 1;
                    },  

                    _ => println!("unknown instruction in 0xF")
                }
            }


            _ => println!("not implementaed yet")
        }

        self.pc += 2;
    }
}

fn parse(c: char) -> u8
{
    match c
    {
        '1' => 1,
        '0' => 0,
            _  => 2,
    }
}

fn apply_scalling(display: &mut Display, ori_x: u16, ori_y: u16, row: u16, column: u16, bite: char) -> bool
{
    let scale = crate::display::SCALE as u16;

    let base_cood_x = ori_x * scale;
    let base_cood_y = ori_y * scale;
    let row = row * scale;
    let column = column * scale;
    let bite_int = parse(bite);

    let mut change_register: bool = false;

    for scale_factor in 0 .. scale
    {
        for scale_factor2 in 0 .. scale
        {
            let cood_x = base_cood_x + scale_factor;
            let cood_y = base_cood_y + scale_factor2;

            let index :u32 = ( (row + cood_y) as u32 * display.get_width() ) + (cood_x + column) as u32;
            let current_pixel = display.is_pixel(index);
            let result = current_pixel ^ bite_int;

            display.write_buffer(index, result as u32 * 0xffffff);

            change_register = current_pixel == 1 && bite_int == 0;
        }
    }

    return change_register;
}