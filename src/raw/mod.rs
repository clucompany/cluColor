
pub mod const_table;


#[macro_export]
///Manual formatting methods (for daily use are not required).
///```
///macro_rules! raw_color {
///	(start) => {"\x1B["}; <-- ascii escape, start format. wait color.
///	(end_color) => {"m"}; <-- color end
///	(bold) => {"\x1B[1m"}; <-- add bold format. wait str. ...
///	(end) => {"\x1B[0m"}; <-- end str
///
///	(b, start) => {b"\x1B["}; <-- slice byte alternative
///	(b, end_color) => {b"m"}; <-- slice byte alternative
///	(b, bold) => {b"\x1B[1m"}; <-- slice byte alternative
///	(b, end) => {b"\x1B[0m"}; <-- slice byte alternative
///}
///```

macro_rules! raw_color {
	(start) => {		"\x1B["	};
	(end_color) => {	"m"		};
	(bold) => {		"\x1B[1m"	};
	(end) => {		"\x1B[0m"	};

	(b, start) => {	b"\x1B["	};
	(b, end_color) => {	b"m"		};
	(b, bold) => {	b"\x1B[1m"	};
	(b, end) => {		b"\x1B[0m"	};
}
