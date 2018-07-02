
use std::fmt::Display;
use std::marker::PhantomData;
use std::fmt;
use std::fmt::Arguments;
use std::io::Write;
use std::io;
use cluColor;

///Lightweight wrap for generalized color type.
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct cluColorWriter<C: cluColor> {
	_phantom: PhantomData<C>,
}

impl<C: cluColor> cluColorWriter<C> {
	#[inline]
	pub fn new() -> cluColorWriter<C> {
		cluColorWriter {
			_phantom: PhantomData,
		}
	}
	
	#[inline(always)]
	pub fn raw_color<'a>(&self) -> &'a str {
		C::raw_color()
	}
	
	#[inline(always)]
	pub fn raw_color_b<'a>(&self) -> &'a [u8] {
		C::raw_color_b()
	}
	
	#[inline(always)]
	pub fn name<'a>(&self) -> &'a str {
		C::name()
	}
	
	#[inline(always)]
	pub fn write<'a>(&self, w: &mut Write, buf: &'a [u8]) -> io::Result<()> {
		C::write(w, buf)
	}
	
	#[inline(always)]
	pub fn write_str<'a>(&self, w: &mut Write, str: &'a str) -> io::Result<()> {
		C::write_str(w, str)
	}
	
	#[inline(always)]
	pub fn write_fmt<'a>(&self, w: &mut Write, fmt: Arguments<'a>) -> io::Result<()> {
		C::write_fmt(w, fmt)
	}
	
	// add /n
	
	#[inline(always)]
	pub fn writen<'a>(&self, w: &mut Write, buf: &'a [u8]) -> io::Result<()> {
		C::writen(w, buf)
	}
	
	#[inline(always)]
	pub fn writen_str<'a>(&self, w: &mut Write, str: &'a str) -> io::Result<()> {
		C::writen_str(w, str)
	}
	
	#[inline(always)]
	pub fn writen_fmt<'a>(&self, w: &mut Write, fmt: Arguments<'a>) -> io::Result<()> {
		C::writen_fmt(w, fmt)
	}
	
	
	#[inline(always)]
	pub fn string_as<'a, A: AsRef<str>>(&self, asref: A) -> String {
		C::string(asref.as_ref())
	}
	
	#[inline(always)]
	pub fn stringn_as<'a, A: AsRef<str>>(&self, asref: A) -> String {
		C::stringn(asref.as_ref())
	}
	
	#[inline(always)]
	pub fn string<'a>(&self, str: &'a str) -> String {
		C::string(str)
	}
	
	#[inline(always)]
	pub fn string_fmt<'a>(&self, fmt: Arguments<'a>) -> String {
		C::string_fmt(fmt)
	}
	
	
	#[inline(always)]
	pub fn stringn<'a>(&self, str: &'a str) -> String {
		C::stringn(str)
	}
	
	#[inline(always)]
	pub fn stringn_fmt<'a>(&self, fmt: Arguments<'a>) -> String {
		C::stringn_fmt(fmt)
	}
}



impl<C: cluColor> Display for cluColorWriter<C> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.name())
	}
}





