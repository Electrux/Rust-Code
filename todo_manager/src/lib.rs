pub mod common;

use std::fs;
use std::process;

pub fn help()
{
	println!( "Help\n\n" );

	println!( "\tOptions:\n" );

	println!( "\t\tadd [ task ]" );
	println!( "\t\t\tAdd a new task to perform" );

	println!( "\t\tshow" );
	println!( "\t\t\tShow all undone tasks and their IDs" );

	println!( "\t\tdone [ task id ]" );
	println!( "\t\t\tFinish and delete a task by its ID which is a number > 1" );

	println!( "\t\thelp" );
	println!( "\t\t\tShow this information" );
}

pub fn insert( args: & Vec< String > )
{
	if args.len() < 3 {
		eprintln!( "Error! Nothing entered to insert..." );
		process::exit( 1 );
	}

	let toinsert = common::combineparams( args, 2, args.len() );
	
	let file = fs::OpenOptions::new()
		.read( true )
		.append( true )
		.create( true )
		.open( common::get_tasks_file() ).unwrap_or_else( | err | {
			eprintln!( "Error occured: {}", err );
			process::exit( 1 );
		} );

	let count = common::count_file_lines( & file );

	common::append_to_file( & file, & toinsert );

	println!( "Created task {} as: {}", count + 1, toinsert );
}

pub fn display()
{
	let file = fs::OpenOptions::new()
		.read( true )
		.create( true )
		.write( true )
		.open( common::get_tasks_file() ).unwrap_or_else( | err | {
			eprintln!( "Error occured: {}", err );
			process::exit( 1 );
		} );

	let lines = common::get_file_lines( & file );

	let mut i: usize = 1;

	let count = lines.len();

	if count <= 0 {
		println!( "No tasks to do! Yaaaaaaaaay!" );
	}
	else {
		println!( "Task ID\t\tTask" );
		for line in lines {
			println!( "{}:\t\t{}", i, line );
			i += 1;
		}
	}
}

pub fn delete( args: & Vec< String > )
{
	if args.len() < 3 {
		eprintln!( "Error! task id to delete is not entered" );
		process::exit( 1 );
	}

	let todel: usize = common::combineparams( args, 2, args.len() ).trim().parse()
		.unwrap_or_else( | err | {
			eprintln!( "Invalid id entered to delete: {}", err );
			process::exit( 1 );
		} );

	let file = fs::OpenOptions::new()
		.read( true )
		.open( common::get_tasks_file() ).unwrap_or_else( | err | {
			eprintln!( "Error occured: {}", err );
			process::exit( 1 );
		} );

	let delline = common::remove_task( file, todel );

	println!( "Deleted task {} as: {}", todel, delline );
}
