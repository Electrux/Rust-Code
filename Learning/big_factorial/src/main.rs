use std::io::Write;

extern crate big_factorial;

fn main()
{
	print!( "Enter a number: " );
	std::io::stdout().flush()
		.expect( "IO Error" );

	let mut val = String::new();
	
	std::io::stdin().read_line( & mut val )
		.expect( "Unable to read line!" );

	let val: usize = val.trim().parse()
		.expect( "Entered data was not a number!" );

	println!( "{}", big_factorial::get_factorial( val ) );
}
