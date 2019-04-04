# cluColor [DEPRECATED, please use https://github.com/clucompany/cluEscSequency]

[![Build Status](https://travis-ci.org/clucompany/cluColor.svg?branch=master)](https://travis-ci.org/clucompany/cluColor)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/cluColor)](https://crates.io/crates/cluColor)
[![Documentation](https://docs.rs/cluColor/badge.svg)](https://docs.rs/cluColor)


Methods for formatted recording of color output.
# Easy methods of formatted recording

	#[macro_use]
	extern crate clucolor;

	let str_colored = color!(blue, "test");
	println!("{}", str_colored);

	let str_colored = color!(blue, bold, "test");
	println!("{}", str_colored);


	let str_colored = color!(bright_red, bold, "test");
	println!("{}", str_colored);

# Generating a string using color types

	#[macro_use]
	extern crate clucolor;

	use clucolor::colors::cluColor;
	use clucolor::colors::BrightRed;

	let string = BrightRed::string_fmt( format_args!("[{:?}] {}", TEST, str) );
	let string = BrightRed::stringn( "color str!" );

# Recording macros in Write trait

	#[macro_use]
	extern crate clucolor;

	use clucolor::colors::Blue;
	use clucolor::colors::BrightBlue;


	writen_color!(&mut ::std::io::stdout(), BrightBlue, "OutValueTest {}", 123);
	writen_color!(&mut ::std::io::stdout(), BrightBlue, "OutValueTest2 {}", 12345);
	writen_color!(&mut File::open("color_file.txt"), BrightBlue, "Color Str:)", 12345);

# Recording using color types

	#[macro_use]
	extern crate clucolor;

	use clucolor::colors::Blue;
	use clucolor::colors::BrightBlue;


	let mut vec: Vec<u8> = Vec::new(); // For Vec implemented Write!!


	let _e = BrightBlue::write_str(&mut vec, "color str!" );

	let _e = vec.write(b"TestValue"); // For Vec implemented Write!!
	//Also this value will remain without color formatting.

	let _e = BrightBlue::writen_str(&mut vec, "end str.." );

	let _e = BrightRed::writen(&mut vec, b"end value.." );

# Use move color arguments

	#[macro_use]
	extern crate clucolor;

	use clucolor::colors::BrightBlue;

	#[derive(Debug, Default)]
	pub struct Items(usize, usize);

	impl Items {
	     #[inline]
	     pub fn count(&self) -> usize {
		  self.0 + self.1
	     }
	}

	let mut item = Items::default();

	for a in 0..15 {
	     BrightGreen::with_color_fmt(format_args!("NUM #{}", a), |fmt_num| {
		  BrightBlue::with_color_fmt(format_args!("ITEM #{:?}", item), |fmt_item| {
		       BrightRed::with_color_fmt(format_args!("ALL_COUNT {}", item.count()), |fmt_count| {
			    println!("{}, {}; {};", fmt_num, fmt_item, fmt_count);
		       });
		  });
	     });

	     item.0 += 1;
	     item.1 += 2;
	}

# Use ColorWriter

	#[macro_use]
	extern crate clucolor;

	use clucolor::colors::Blue;

	let writer = Blue::writer();

	let stdout = ::std::io::stdout();
	let mut lock_stdio = stdout.lock();

	writer.writen(&mut lock_stdio, b"TestWriten").unwrap();

	All other functions are implemented in color mod with the help of cluColor!


# License

Copyright 2018 #UlinProject Денис Котляров

Licensed under the Apache License, Version 2.0
