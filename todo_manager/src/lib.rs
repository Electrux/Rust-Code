pub mod common;

use std::fs;
use std::io;
use std::io::BufRead;
use std::process;
//use std::prelude::*;

pub fn insert( args: & Vec< String > )
{
	if args.len() < 3 {
		println!( "Error! Nothing entered to insert..." );
		process::exit( 1 );
	}

	let toinsert = common::combineparams( args, 2, args.len() );

	println!( "To insert: {}", toinsert );

	let file = fs::File::open( "test.txt" ).unwrap_or_else( | err | {
		println!( "Error occured: {}", err );
		process::exit( 1 );
	} );

	let filereader = io::BufReader::new( & file );

	for ( num, line ) in filereader.lines().enumerate() {
		let line = line.unwrap();
		println!( "{}: {}", num, line );
	}
}
