fn fun1(arg:& String) {
	println!("arg = {}", arg);
}
fn fun2(arg:& String) {
	println!("arg = {}", arg);
}


fn main() {
	let s:String = String::from("Hello"); // s meta: Stack, raw : heap
	fun1(&s);	
	println!("s = {}", s); // 
	fun2(&s);	
}
