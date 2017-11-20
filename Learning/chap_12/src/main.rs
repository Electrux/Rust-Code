extern crate chap_12;

use chap_12::Config;

fn main()
{
	let args: Vec< String > = std::env::args().collect();

	let config = Config::new( & args, 2 ).unwrap_or_else( | err | {
		eprintln!( "Problem parsing arguments: {}", err );
		std::process::exit( 1 );
	} );

	if let Err( e ) = chap_12::run( & config ) {
		eprintln!( "Application error: {}", e );
		std::process::exit( 1 );
	}
}
