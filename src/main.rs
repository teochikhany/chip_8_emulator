mod ram;

use ram::Ram;

fn main()
{
    println!("test");

    let mut test = Ram::new();

    let result = test.read(0, 10);
    println!("{:?}", result);

    test.write(8, 1, &vec![5]);

    let result = test.read(0, 10);
    println!("{:?}", result);
}

