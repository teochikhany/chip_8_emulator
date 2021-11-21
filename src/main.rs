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

    ram.write_vec(0x200, &data);

    println!("{:?}", ram.read(0x200, 10));

    let mut t = 0;
    while cpu.pc < 0x200 + data.len() as u16
    {
        cpu.run_instruction(&mut ram);
        cpu.pc = cpu.pc + 2;
        t += 2;
    }   
    println!("t is: {}", t);
    println!("data is: {}", data.len());
    println!("pc is: {}", cpu.pc);

    println!("{:?}", ram.read(0x200 + data.len() as u16 - 50, 100));
}


fn test()
{
    println!("test");

    let mut test = Ram::new();

    println!("{:?}", test.read(0, 10));

    test.write(8, 1, &vec![5]);
    
    println!("{:?}", test.read(0, 10));

    let mut test_cpu = Cpu::new();
    test_cpu.debug();

    test_cpu.write_register(5, 7);

    test_cpu.debug();

    println!("value at register {}: {:}", 5, test_cpu.read_register(5));
}
