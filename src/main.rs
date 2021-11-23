mod ram;
mod cpu;
mod display;

use std::env;
use std::fs::File;
use std::io::Read;
use std::thread;

use ram::Ram;
use cpu::Cpu;
use display::Display;
use std::time::{Duration, Instant};


const duration: std::time::Duration = Duration::from_millis(100);

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


    while display.is_open()
    {
        if Instant::now() - time > duration
        {
            cpu.run_instruction(&mut ram, &mut display);
            cpu.substract_dt();

            time = Instant::now();
        }
        else
        {
            thread::sleep(duration);
        }
    }   
}

