//Copyright 2018 #UlinProject Денис Котляров

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//       http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.


//#Ulin Project 1718
//

/*!
Methods for formatted recording of color output.

# Easy methods of formatted recording

```
#[macro_use]
extern crate clucolor;

let str_colored = color!(blue, "test");
println!("{}", str_colored);
	
let str_colored = color!(blue, bold, "test");
println!("{}", str_colored);

	
let str_colored = color!(bright_red, bold, "test");
println!("{}", str_colored);
```

# Generating a string using color types

```
#[macro_use]
extern crate clucolor;

use clucolor::colors::cluColor;
use clucolor::colors::BrightRed;

let string = BrightRed::string_fmt( format_args!("[{:?}] {}", TEST, str) );
let string = BrightRed::stringn( "color str!" );

```


# Recording macros in `Write trait`

```
#[macro_use]
extern crate clucolor;

use clucolor::colors::Blue;
use clucolor::colors::BrightBlue;


writen_color!(&mut ::std::io::stdout(), BrightBlue, "OutValueTest {}", 123);
writen_color!(&mut ::std::io::stdout(), BrightBlue, "OutValueTest2 {}", 12345);
writen_color!(&mut File::open("color_file.txt"), BrightBlue, "Color Str:)", 12345);
```

# Recording using color types

```
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

```

All other functions are implemented in color mod with the help of cluColor!

*/


#[macro_use]
///Manual methods for color formatting.
pub mod raw;

///Generalized types of colors.
pub mod colors;

///Additional methods of color recording.
pub mod writer;


macro_rules! build_colored {
	( $(  $color:tt | $name:ident | $name2:ident )+ ) => {
		
		///Concat macro for color generation.
		///```	
		///DATA | NAME_COLOR		| NAME2_COLOR
		///---------------------------------------------
		///"30" | black			| BLACK
		///"31" | red			| RED
		///"32" | green			| GREEN
		///"33" | yellow			| YELLOW
		///"34" | blue			| BLUE
		///"35" | magenta			| MAGENTA
		///"36" | cyan			| CYAN
		///"37" | white			| WHITE
		///
		///"90" | bright_black		| BRIGHT_BLACK
		///"91" | bright_red		| BRIGHT_RED
		///"92" | bright_green		| BRIGHT_GREEN
		///"93" | bright_yellow		| BRIGHT_YELLOW
		///"94" | bright_blue		| BRIGHT_BLUE
		///"95" | bright_magenta		| BRIGHT_MAGENTA
		///"96" | bright_cyan		| BRIGHT_CYAN
		///"97" | bright_white		| BRIGHT_WHITE
		///```
		///
		///```
		///let str_colored = color!(blue, bold, "test");
		///println!("{}", str_colored);
		///```
		
		#[macro_export]
		macro_rules! color_args {
			$(
				
				($name, $s:expr) => {
					format_args!(
						"{}{}{}",
						
						concat!(
							raw_color!(start),
							$color,
							raw_color!(end_color),
						), 
						$s, 
						raw_color!(end),
					)
				};
				
				($name, bold, $s:expr) => {
					format_args!(
						"{}{}{}{}",
						
						concat!(
							raw_color!(start),
							$color,
							raw_color!(end_color),
						), 
						raw_color!(bold), 
						$s, 
						raw_color!(end),
					)
				};
				
				($name2, $s:expr) => { color_args!($name, $s) };
				($name2, bold, $s:expr) => { color_args!($name, bold, $s) };
				
			)+
		}
		
		///A concatenated macro for generating a colored static string.
		#[macro_export]
		macro_rules! color {
			$(
				
				($name, $s:expr) => {
					concat!(						
						concat!(
							raw_color!(start),
							$color,
							raw_color!(end_color),
						), 
						$s, 
						raw_color!(end),
					)
				};
				
				($name, bold, $s:expr) => {
					concat!(						
						concat!(
							raw_color!(start),
							$color,
							raw_color!(end_color),
						), 
						raw_color!(bold), 
						$s, 
						raw_color!(end),
					)
				};
				
				($name2, $s:expr) => { color_args!($name, $s) };
				($name2, bold, $s:expr) => { color_args!($name, bold, $s) };
				
			)+
		}
		
		
		///A concatenated macro for creating a color dynamic string.
		///```	
		///DATA | NAME_COLOR		| NAME2_COLOR
		///---------------------------------------------
		///"30" | black			| BLACK
		///"31" | red			| RED
		///"32" | green			| GREEN
		///"33" | yellow			| YELLOW
		///"34" | blue			| BLUE
		///"35" | magenta			| MAGENTA
		///"36" | cyan			| CYAN
		///"37" | white			| WHITE
		///
		///"90" | bright_black		| BRIGHT_BLACK
		///"91" | bright_red		| BRIGHT_RED
		///"92" | bright_green		| BRIGHT_GREEN
		///"93" | bright_yellow		| BRIGHT_YELLOW
		///"94" | bright_blue		| BRIGHT_BLUE
		///"95" | bright_magenta		| BRIGHT_MAGENTA
		///"96" | bright_cyan		| BRIGHT_CYAN
		///"97" | bright_white		| BRIGHT_WHITE
		///```
		#[macro_export]
		macro_rules! color_format {
			$(
				($name, $s:expr) => {
					format!("{}", color_args!($name, $s))
				};
				($name2, $s:expr) => { color!($name, $s) };
				
				($name, bold, $s:expr) => {
					format!("{}", color_args!($name, bold, $s))
				};
				($name2, bold, $s:expr) => { color!($name, bold, $s) };	
			)+
		}
		
	}
}

build_colored! (
	"30" |	black			| BLACK
	"31" |	red			| RED
	"32" |	green			| GREEN
	"33" |	yellow			| YELLOW
	"34" |	blue			| BLUE
	"35" |	magenta		| MAGENTA
	"36" |	cyan			| CYAN
	"37" |	white			| WHITE
	
	"90" |	bright_black		| BRIGHT_BLACK
	"91" |	bright_red		| BRIGHT_RED
	"92" |	bright_green		| BRIGHT_GREEN
	"93" |	bright_yellow		| BRIGHT_YELLOW
	"94" |	bright_blue		| BRIGHT_BLUE
	"95" |	bright_magenta	| BRIGHT_MAGENTA
	"96" |	bright_cyan		| BRIGHT_CYAN
	"97" |	bright_white		| BRIGHT_WHITE
);


use std::fmt::Arguments;
use std::io::Write;
use writer::cluColorWriter;
use std::hash::Hash;
use std::fmt::Display;
use std::fmt::Debug;
use std::io;

///Common features implemented by the generalized type.
#[allow(non_camel_case_types)]
pub trait cluColor: Clone + Debug + Display + Eq + Hash + Ord + PartialEq + PartialOrd {
	///Color str type
	#[inline(always)]
	fn raw_color<'a>() -> &'a str;
	
	///Color array type
	#[inline(always)]
	fn raw_color_b<'a>() -> &'a [u8];
	
	///Name color
	#[inline(always)]
	fn name<'a>() -> &'a str;
	
	#[inline]
	fn writer<'a>() -> cluColorWriter<Self> {
		cluColorWriter::<Self>::new()
	}
	
	
	#[inline]
	fn string_as<'a, A: AsRef<str>>(asref: A) -> String {
		Self::string(asref.as_ref())
	}
	#[inline]
	fn stringn_as<'a, A: AsRef<str>>(asref: A) -> String {
		Self::stringn(asref.as_ref())
	}
	
	
	#[inline]
	fn string<'a>(str: &'a str) -> String;
	
	#[inline]
	fn string_fmt<'a>(fmt: Arguments<'a>) -> String;
	
	
	#[inline]
	fn stringn<'a>(str: &'a str) -> String;
	
	#[inline]
	fn stringn_fmt<'a>(fmt: Arguments<'a>) -> String;
	
	
	#[inline]
	fn write_as<'a, A: AsRef<[u8]>>(w: &mut Write, asref: A) -> io::Result<()> {
		Self::write(w, asref.as_ref())
	}
	#[inline]
	fn writen_as<'a, A: AsRef<[u8]>>(w: &mut Write, asref: A) -> io::Result<()> {
		Self::writen(w, asref.as_ref())
	}
	
	#[inline]
	fn write<'a>(w: &mut Write, buf: &'a [u8]) -> io::Result<()>;
	
	#[inline]
	fn write_str<'a>(w: &mut Write, str: &'a str) -> io::Result<()>;
	
	#[inline]
	fn write_fmt<'a>(w: &mut Write, fmt: Arguments<'a>) -> io::Result<()>;
	
	// n methods
	
	#[inline]
	fn writen<'a>(w: &mut Write, buf: &'a [u8]) -> io::Result<()>;
	
	#[inline]
	fn writen_str<'a>(w: &mut Write, str: &'a str) -> io::Result<()>;
	
	#[inline]
	fn writen_fmt<'a>(w: &mut Write, fmt: Arguments<'a>) -> io::Result<()>;
}




#[macro_export]
///Macro of the formatted entry in the trait.
macro_rules! write_color {
	( $write:expr, $color:tt, $( $arg:tt )* ) => {
		$color::write_fmt($write, format_args!( $( $arg )* ))
	};
}


#[macro_export]
///Macro of the formatted entry in the trait. Adds /n to end.
macro_rules! writen_color {
	( $write:expr, $color:tt, $( $arg:tt )* ) => {
		$color::writen_fmt($write, format_args!( $( $arg )* ))
	};
}





//Ulin Project 1817
