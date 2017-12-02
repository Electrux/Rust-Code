//use std::io;
//use std::io::Write;
use std::process;
use std::env;
extern crate todo_manager;

fn main()
{
	let args: Vec< String > = env::args().collect();

	if args.len() < 2 {
		eprintln!( "Use: {} help - for more information on this utility", args[ 0 ] );
		process::exit( 1 );
	}

	match args[ 1 ].trim() {
		"add" => todo_manager::insert( & args ),
		"done" => todo_manager::delete( & args ),
		"show" => todo_manager::display(),
		"help" => todo_manager::help(),
		_ => {
			eprintln!( "Invalid parameter. Use {} help for information", args[ 0 ] );
			process::exit( 1 );
		},
	}
}


