struct myVec {
	len:u32,
	capacity:u32,
}

fn main()
{
	//Vec::new();
	let v:Vec<i32> = vec![1,2,3,4]; // non primitive data type
	println!("v = {:?}", v); // To call debug function from Vector::Debug()

	println!("v[0] = {}", v[0]);
	println!("v = {:?}", v); // To call debug function from Vector::Debug()
}
