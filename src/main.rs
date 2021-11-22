mod ram;
mod cpu;

use ram::Ram;
use cpu::Cpu;

use std::env;
use std::fs::File;
use std::io::Read;


fn main()
{

    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).unwrap();

    let mut file = File::open(format!("data/{}", file_name)).expect("File not found!");
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("File not found!");

    let mut ram = Ram::new();
    let mut cpu = Cpu::new();

    ram.write(0x200, &data);

    println!("{:?}", ram.read(0x200, 10));

    while cpu.pc < 0x200 + data.len() as u16
    {
        cpu.run_instruction(&mut ram);
    }   

}

