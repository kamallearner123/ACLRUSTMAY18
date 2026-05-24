fn analyse_update(data:&mut String) 
{
	data.push_str("......");
	println!("data = {}", data);
}

fn main() {
	let mut s1:String = String::from("Hello");
	analyse_update(&mut s1);
	println!("s1 = {}",s1);

	let s2 = &mut s1;
	s2.push_str("!!!!");
	println!("s2 = {}", s2);		

	println!("s1 = {}", s1);
	println!("s2 = {}", s2);		
}
