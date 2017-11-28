//use std::io;
//use std::io::Write;
use std::env;
extern crate todo_manager;

fn main()
{
	let args: Vec< String > = env::args().collect();

	if args.len() < 2 {
		println!( "Usage: {} [ task ] [ args ]", args[ 0 ] );
	}

	match args[ 1 ].trim() {
		"insert" => todo_manager::insert( & args ),
		_ => println!( "Invalid parameter. Use {} help for information", args[ 0 ] ),
	}
}


