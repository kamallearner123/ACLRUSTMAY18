fn main() {
	let mut v1:Vec<u32> = vec![1, 10];
	println!("v1 = {:?}", v1);
	v1.push(100);
	println!("v1 = {:?}", v1);

	for i in 0..10 {
		v1.push(i+200);
	}
	println!("v1 = {:?}", v1);

	let v2 = v1.clone();
	println!("v2 = {:?}", v2);		

	let a1 = [1,2,3];
	for i in a1 {
		println!("i={}", i);


		if i>2 {
		}
	}

	// for/while .... while(True)
	
}
