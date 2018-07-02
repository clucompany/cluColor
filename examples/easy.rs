
#[macro_use]
extern crate clucolor;

pub fn main() {
	
	let str_colored = color!(blue, "test");
	println!("{}", str_colored);
	
	let str_colored = color!(blue, bold, "test");
	println!("{}", str_colored);
	
	
	let str_colored = color!(bright_blue, bold, "test");
	println!("{}", str_colored);
}
