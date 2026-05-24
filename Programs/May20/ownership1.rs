fn fun1(arg:String) {
	println!("arg = {}", arg);
}

fn main() {
	let s:String = String::from("Hello"); // s meta: Stack, raw : heap
	let _t = s.clone(); //Ownership moved from s to t
	println!("s = {}", s);
}
