
extern crate clucolor;

use clucolor::cluColor;
use clucolor::colors::BrightRed;
use std::marker::PhantomData;
use std::time::SystemTime;

fn main() {
	
	let time_start = SystemTime::now();
	println!("Start time {:?} nanos", time_start.elapsed().unwrap().subsec_nanos());
	
	let writer = MyWriter::<BrightRed>::new(time_start);
	writer.println("OK");
	writer.println("OK12");
	writer.println("OK123");
}


#[derive(Debug)]
pub struct MyWriter<C: cluColor> {
	_c: PhantomData<C>,
	time: SystemTime,
}

impl<C: cluColor> MyWriter<C> {
	#[inline]
	pub fn new(time: SystemTime) -> MyWriter<C> {
		MyWriter {
			_c: PhantomData,
			time: time,
		}
	}
	
	#[inline]
	pub fn string<'a>(&self, str: &'a str) -> String {
		C::string_fmt( format_args!("[{:?} nanos] {}", SystemTime::now().elapsed().unwrap().subsec_nanos(), str) )
	}
	
	pub fn println<'a>(&self, str: &'a str) {
		println!("{}", self.string(str));
	}
}



