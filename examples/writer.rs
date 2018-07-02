
#[macro_use]
extern crate clucolor;

use clucolor::colors::Blue;
use clucolor::colors::BrightBlue;
use clucolor::cluColor;

fn main() {
	{
		let writer = Blue::writer();
	
		let stdout = ::std::io::stdout();
		let mut lock_stdio = stdout.lock();
	
		writer.writen(&mut lock_stdio, b"TestWriten");
	}
	{
		let writer = Blue::writer();
	
		let stdout = ::std::io::stdout();
		let mut lock_stdio = stdout.lock();
	
		write_color!(&mut lock_stdio, BrightBlue, "{} {} macro write...", 12345, "str");
	}
	
	{
		writen_color!(&mut ::std::io::stdout(), BrightBlue, "{} {}", 123, "str");
		writen_color!(&mut ::std::io::stdout(), BrightBlue, "{} {}", 12345, "str");
	}
}


