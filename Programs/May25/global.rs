static mut counter:i32 = 0;


fn main() {
	unsafe {
		println!("counter = {}", counter);
	}
}
