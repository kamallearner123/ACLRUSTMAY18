
#[derive(Debug)]
struct stu {
	roll:i32,
	sec:i8
}

fn main()
{
	let mut a:Box<stu> = Box::new(stu{roll:10, sec:1});
	a.roll = 100;
	println!("a = {:?}",a);
}
