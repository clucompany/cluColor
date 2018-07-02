
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::io::Write;
use std::fmt::Arguments;
use std::io;

use cluColor;


macro_rules! build_type_colored {
	
	( $(  $color:tt | $color_byte:tt | $name:ident )+ ) => {
		build_type_colored!( $( $color | $color_byte | $name, stringify!($name) )+ );
	};
	

	
	( $(  $color:tt | $color_byte:tt | $name:ident, $doc_name:expr )+ ) => {
		$(
			#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
			
			#[doc = "Color Type `"]
			#[doc = $doc_name]
			#[doc = "`."]
			pub enum $name {}
			
			
			impl cluColor for $name {
				
				#[doc = "Return \""]
				#[doc = $color]
				#[inline(always)]
				fn raw_color<'a>() -> &'a str {
					$color
				}
				
				#[doc = "Return b\""]
				#[doc = $color]
				#[doc = "\""]
				#[inline(always)]
				fn raw_color_b<'a>() -> &'a [u8] {
					$color_byte
				}
				
				
				#[doc = "Return \""]
				#[doc = $doc_name]
				#[doc = "\""]
				#[inline(always)]
				fn name<'a>() -> &'a str {
					$doc_name
				}
				
				#[inline]
				fn write<'a>(w: &mut Write, array: &'a [u8]) -> io::Result<()> {
					write!(w, "{}{}{}{}{}",
						
							raw_color!(start),
							Self::raw_color(),
							raw_color!(end_color),
						unsafe { ::std::str::from_utf8_unchecked(array) }, 
						raw_color!(end)
					)
				}
				
				#[inline]
				fn write_str<'a>(w: &mut Write, str: &'a str) -> io::Result<()> {
					write!(w, "{}{}{}{}{}",
						
							raw_color!(start),
							Self::raw_color(),
							raw_color!(end_color),
						str, 
						raw_color!(end)
					)
				}
				
				#[inline]
				fn write_fmt<'a>(w: &mut Write, fmt: Arguments<'a>) -> io::Result<()> {
					write!(w, "{}{}{}{}{}",
							raw_color!(start),
							Self::raw_color(),
							raw_color!(end_color),
						fmt, 
						raw_color!(end)
					)
				}
				
				// add n
				
				#[inline]
				fn writen<'a>(w: &mut Write, array: &'a [u8]) -> io::Result<()> {
					write!(w, "{}{}{}{}{}\n",
						
							raw_color!(start),
							Self::raw_color(),
							raw_color!(end_color),
						unsafe { ::std::str::from_utf8_unchecked(array) }, 
						raw_color!(end)
					)
				}
				
				#[inline]
				fn writen_str<'a>(w: &mut Write, str: &'a str) -> io::Result<()> {
					write!(w, "{}{}{}{}{}\n",
						
							raw_color!(start),
							Self::raw_color(),
							raw_color!(end_color),
						str, 
						raw_color!(end)
					)
				}
				
				#[inline]
				fn writen_fmt<'a>(w: &mut Write, fmt: Arguments<'a>) -> io::Result<()> {
					write!(w, "{}{}{}{}{}\n",
							raw_color!(start),
							Self::raw_color(),
							raw_color!(end_color),
						fmt, 
						raw_color!(end)
					)
				}
				
				
				#[inline]
				fn string_fmt<'a>(fmt: Arguments<'a>) -> String {
					format!("{}{}{}{}{}",
							raw_color!(start),
							Self::raw_color(),
							raw_color!(end_color),
						fmt, 
						raw_color!(end)
					)
				}
				
				#[inline]
				fn string<'a>(str: &'a str) -> String {
					format!("{}{}{}{}{}",
							raw_color!(start),
							Self::raw_color(),
							raw_color!(end_color),
						str, 
						raw_color!(end)
					)
				}
				
				#[inline]
				fn stringn_fmt<'a>(fmt: Arguments<'a>) -> String {
					format!("{}{}{}{}{}\n",
							raw_color!(start),
							Self::raw_color(),
							raw_color!(end_color),
						fmt, 
						raw_color!(end)
					)
				}
				
				#[inline]
				fn stringn<'a>(str: &'a str) -> String {
					format!("{}{}{}{}{}\n",
							raw_color!(start),
							Self::raw_color(),
							raw_color!(end_color),
						str, 
						raw_color!(end)
					)
				}
				
			}
			
			impl Display for $name {
				fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
					write!(f, "{}", Self::name())
				}
			}
			
			impl Debug for $name {
				fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
					f.debug_struct(Self::name()).finish()
				}
			}
			
		)+

	};
}

//Generation of colors at compile time
build_type_colored! (
	"30" |		b"30"		|	Black
	"31" |		b"31"		|	Red
	"32" |		b"32"		|	Green
	"33" |		b"33"		|	Yellow
	"34" |		b"34"		|	Blue
	"35" |		b"35"		|	Magenta
	"36" |		b"36"		|	Cyan
	"37" |		b"37"		|	White
	
	/*"30" |		b"30"		|	BoldBlack
	"31" |		b"31"		|	BoldRed
	"32" |		b"32"		|	BoldGreen
	"33" |		b"33"		|	BoldYellow
	"34" |		b"34"		|	BoldBlue
	"35" |		b"35"		|	BoldMagenta
	"36" |		b"36"		|	BoldCyan
	"37" |		b"37"		|	BoldWhite*/
	
	
	"90" |		b"90"		|	BrightBlack
	"91" |		b"91"		|	BrightRed
	"92" |		b"92"		|	BrightGreen
	"93" |		b"93"		|	BrightYellow
	"94" |		b"94"		|	BrightBlue
	"95" |		b"95"		|	BrightMagenta
	"96" |		b"96"		|	BrightCyan
	"97" |		b"97"		|	BrightWhite
	
	/*"90" |		b"90"		|	BoldBrightBlack
	"91" |		b"91"		|	BoldBrightRed
	"92" |		b"92"		|	BoldBrightGreen
	"93" |		b"93"		|	BoldBrightYellow
	"94" |		b"94"		|	BoldBrightBlue
	"95" |		b"95"		|	BoldBrightMagenta
	"96" |		b"96"		|	BoldBrightCyan
	"97" |		b"97"		|	BoldBrightWhite*/
);

