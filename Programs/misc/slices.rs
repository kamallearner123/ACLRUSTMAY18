#[derive(Debug)]
struct stu {
	a:i32, b:i32
}

fn main() {
	let s:Vec<stu> = vec![stu{a:10,b:20}];
	println!("s = {:?}", s);


	let num = s[0];
	//println!("s = {:?}", s);
	
}	
