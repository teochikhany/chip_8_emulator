mod ram;
mod cpu;

use ram::Ram;
use cpu::Cpu;


fn main()
{
    println!("test");

    let mut test = Ram::new();

    println!("{:?}", test.read(0, 10));

    test.write(8, 1, &vec![5]);
    
    println!("{:?}", test.read(0, 10));

    let test_cpu = Cpu::new();
    test_cpu.debug();
}

