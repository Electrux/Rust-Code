
fn main()
{	
	let mut first: u64 = 0;
	let mut second: u64 = 1;
	let mut third: u64;

	let mut endpoint = String::new();

	std::io::stdin().read_line( & mut endpoint )
		.expect( "Read line error" );

	let endpoint: u64 = endpoint.trim().parse()
		.expect( "Invalid number entered! Aborting!!" );

	for _ in 1 .. endpoint {

		print!( "{} ", first );

		third = first + second;
		first = second;
		second = third;
	}

	println!( "" );
}
