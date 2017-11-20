use std::io::Read;

fn main()
{
	let f = read_file_str( "test.txt" )
		.expect( "Unable to open file!" );
}


fn read_file_str( filename: & str ) -> Result< String, std::io::Error >
{
	let mut s = String::new();
	
	std::fs::File::open( & filename )?.read_to_string( & mut s )?;

	Ok( s )
}
