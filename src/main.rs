mod ram;

use ram::Ram;

fn main()
{
    println!("test");

    let test = Ram::new();

    let result = test.read(0, 100);

    println!("{:?}", result);
}

