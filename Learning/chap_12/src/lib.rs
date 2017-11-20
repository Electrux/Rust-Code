use std::io::prelude::*;

#[cfg(test)]
mod test
{
	use super::*;

	static CONTENTS: &'static str = "\
Rust:
Safe, Fast, Productive
Pick three.";
	
	#[test]
	fn case_sensitive()
	{
		let query = "duct";

		assert_eq!( vec![ "Safe, Fast, Productive" ], search( query, CONTENTS ) );
	}

	#[test]
	fn case_insensitive()
	{
		let query = "rust";

		assert_eq!( vec![ "Rust:" ], search_insensitive( query, CONTENTS ) );
	}
}

pub fn search< 'a >( query: & str, contents: &'a str ) -> Vec< &'a str >
{
	let mut results = Vec::new();

	for line in contents.lines() {

		if line.contains( query ) {

			results.push( line );
		}
	}

	results
}

pub fn search_insensitive< 'a >( query: & str, contents: &'a str ) -> Vec< &'a str >
{
	let query = query.to_lowercase();
	
	let mut results = Vec::new();

	for line in contents.lines() {

		if line.to_lowercase().contains( & query ) {

			results.push( line );
		}
	}

	results
}

pub fn run( config: & Config ) -> Result< (), Box< std::error::Error > >
{
	let mut file = std::fs::File::open( config.filename )
		.unwrap_or_else( | err | {
			eprintln!( "Unable to open file: {}.\n\tError: {}",
				   config.filename, err );
			std::process::exit( 1 );
		} );

	let mut contents = String::new();

	file.read_to_string( & mut contents )?;

	let lines;
	
	if config.case_sensitive {
		lines = search( config.query, & contents );
	}
	else {
		lines = search_insensitive( config.query, & contents );
	}

	for line in & lines {
		println!( "Match: {}", line );
	}
	
	if lines.len() == 0 {
		eprintln!( "Nothing found!" );
	}

	Ok( () )
}

pub struct Config< 'a >
{
	pub query: &'a String,
	pub filename: &'a String,
	pub case_sensitive: bool,
}

impl< 'a > Config< 'a >
{
	pub fn new( args: & [ String ], minargs: usize ) -> Result< Config, String >
	{
		if args.len() < minargs + 1 {

			return Err( format!( "Not enough arguments! Expected: {}, Received: {}", minargs, args.len() - 1 ) );
		}
		
		let query = & args[ 1 ];
		let filename = & args[ 2 ];
		let case_sensitive = std::env::var( "CASE_INSENSITIVE" ).is_err();

		Ok( Config{ query: query, filename: filename, case_sensitive: case_sensitive } )
	}
}
