use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::process;
use std::env;

pub const FILENAME: & str = ".tasks";

pub fn get_tasks_file() -> String
{
	let homedir = env::var( "HOME" )
		.unwrap_or_else( | err | {
			eprintln!( "Unable to fetch home directory from environment variable {}",
				   err );
			process::exit( 1 );
		} );

	let file = homedir + "/" + FILENAME;

	file
}

pub fn combine_params( args: & Vec< String >, from: usize, to: usize ) -> String
{
	if from >= to {
		return String::new()
	}

	let mut combined: String = args[ from ].clone();

	for i in from + 1 .. to {
		combined = combined + " ";
		combined = combined + & args[ i ];
	}

	combined
}

pub fn get_file_lines( file: & fs::File ) -> Vec< String >
{
	let filereader = io::BufReader::new( file );

	let mut lines = Vec::new();

	for ( _, line ) in filereader.lines().enumerate() {
		lines.push( line.unwrap() );
	}

	lines
}

pub fn count_file_lines( file: & fs::File ) -> usize
{
	let filereader = io::BufReader::new( file );

	filereader.lines().count()
}

pub fn append_to_file( file: & fs::File, line: & str )
{
	let mut filewriter = io::BufWriter::new( file );

	let data = line.to_owned() + "\n";

	filewriter.write( data.as_bytes() )
		.unwrap_or_else( | err | {
			eprintln!( "Unable to write to file: {}", err );
			process::exit( 1 );
		} );
}

pub fn remove_task( mut file: fs::File, num: usize ) -> String
{
	let lines = get_file_lines( & file );
	let count = lines.len();

	let num = num - 1;

	if num >= count {
		eprintln!( "ID entered is too large or there is no task left to remove" );
		process::exit( 1 );
	}

	file = del_and_recreate( file );

	let mut i: usize = 0;

	let mut removedline = String::new();
	for line in lines {
		if i == num {
			removedline = line;
		}
		else {
			append_to_file( & file, & line );
		}
		i += 1;
	}

	removedline
}

pub fn switch_tasks( mut file: fs::File, t1: usize, t2: usize )
{
	let mut lines = get_file_lines( & file );
	let count = lines.len();

	let t1 = t1 - 1;
	let t2 = t2 - 1;

	if t1 >= count {
		eprintln!( "First ID entered is too large or there is no task left in list" );
		process::exit( 1 );
	}

	if t2 >= count {
		eprintln!( "Second ID entered is too large or there is no task left in list" );
		process::exit( 1 );
	}

	file = del_and_recreate( file );

	let tempstr = lines[ t1 ].clone();
	lines[ t1 ] = lines[ t2 ].clone();
	lines[ t2 ] = tempstr;

	for line in lines {
		append_to_file( & file, & line );
	}
}

pub fn del_and_recreate( file: fs::File ) -> fs::File
{
	drop( file );

	fs::remove_file( get_tasks_file() )
		.unwrap_or_else( | err | {
			eprintln!( "Unable to delete file: {}", err );
			process::exit( 1 );
		} );

	fs::OpenOptions::new()
		.read( true )
		.append( true )
		.create( true )
		.open( get_tasks_file() ).unwrap_or_else( | err | {
			eprintln!( "Error occured: {}", err );
			process::exit( 1 );
		} )
}
