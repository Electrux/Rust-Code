use std::io::Write;

extern crate down_to_nine;

fn main()
{
	print!( "Enter a number: " );
	std::io::stdout().flush()
		.expect( "Unable to flush std out!" );

	let mut num = String::new();

	std::io::stdin().read_line( & mut num )
		.expect( "Unable to read line!" );

	let num: u64 = num.trim().parse()
		.expect( "You did not enter a number!" );

	down_to_nine::run( num );
}
