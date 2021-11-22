mod ram;
mod cpu;
mod display;

use std::env;
use std::fs::File;
use std::io::Read;

use ram::Ram;
use cpu::Cpu;
use display::Display;
use std::time::{Duration, Instant};


fn main()
{
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).unwrap();

    let mut file = File::open(format!("data/{}", file_name)).expect("File not found!");
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("File not found!");

    let mut ram     = Ram::new();
    let mut cpu     = Cpu::new();
    let mut display = Display::new();

    ram.write(0x200, &data);

    let mut time = Instant::now();

    // while cpu.pc < 0x200 + data.len() as u16
    loop
    {
        cpu.run_instruction(&mut ram, &mut display);

        if Instant::now() - time > Duration::from_millis(10)
        {
            time = Instant::now();
            cpu.substract_dt();
        }
    }   
}

