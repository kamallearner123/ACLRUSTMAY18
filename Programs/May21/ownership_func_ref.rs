fn reader(data:&String) { //READ-ONLY 
	data.push_str(".........");
	println!("data = {}", data);
}

fn print(num:i8)
{
	println!("num = {}", num);
}

fn main() {
	let s1: String = String::from("Hello");
	reader(&s1);
	println!("s1 = {}", s1);

	let n:i8 = 10; // primitive
	let t = n;
	print(n); 
	println!("Main: n = {}", n);
}
