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

    let mut test_cpu = Cpu::new();
    test_cpu.debug();

    test_cpu.write_register(5, 7);

    test_cpu.debug();

    println!("value at register {}: {:}", 5, test_cpu.read_register(5));
}

