/*
struct stu {
	int id;
	int pin;
};*/

#[derive(Debug)]
struct stu {
	id:i32, sec:char, name:String
}
	

fn main() 
{
	let mut v1 = vec![stu{id:10, sec:'A', name:String::from("kamal")}]; //Rust can inference data type//

	// pop, insert, push, sort, reverse, len, capacity
	v1.pop();

	let num = v1[0];
	
	println!("v1 is {:?}", v1[0].id);// ERROR
}	
	
