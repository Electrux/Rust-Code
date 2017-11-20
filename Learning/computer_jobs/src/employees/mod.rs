use std;
use std::io::Write;

pub mod employee;

use self::employee::*;

static mut EMP_COUNT: u64 = 0;

pub struct Employees
{
	employees: Vec< Employee >,
}

impl Employees
{
	pub fn new() -> Employees
	{
		let emps = Employees{ employees: Vec::new() };

		emps
	}
	
	pub unsafe fn create_new_employee( & mut self ) -> Result< u64, String >
	{
		let mut name = String::new();
		let mut title = String::new();
		
		print!( "Enter employee name: " );
		std::io::stdout().flush()
			.expect( "IO not working correctly" );
		
		std::io::stdin().read_line( & mut name )
			.expect( "Unable to read line" );;

		name = name.trim().to_string();
		title = title.trim().to_string();

		print!( "Enter employee title: " );
		std::io::stdout().flush()
			.expect( "IO not working correctly" );
		
		std::io::stdin().read_line( & mut title )
			.expect( "Unable to read line" );
		
		let emp = Employee::new( & name, & title, EMP_COUNT );

		self.employees.push( emp );

		EMP_COUNT += 1;

		Ok( EMP_COUNT )
	}

	pub unsafe fn delete_employee( & mut self ) -> Result< u64, String >
	{
		let mut id_str = String::new();
		
		print!( "Enter employee id: " );
		std::io::stdout().flush()
			.expect( "IO not working correctly" );
		
		std::io::stdin().read_line( & mut id_str )
			.expect( "Unable to read line" );

		let id: i64 = id_str.trim().parse()
			.expect( "Needed a number" );

		if id < 0 || id >= self.employees.len() as i64 {
			return Err( "Number entered is negative or too large!".to_string() );
		}
		
		self.employees.remove( id as usize );

		EMP_COUNT -= 1;

		Ok( EMP_COUNT )
	}

	pub fn display_all( & self )
	{
		println!( "------------------------------------------" );
		
		for emp in self.employees.iter() {
			println!( "------------------------------" );
			emp.display();
			println!( "------------------------------" );
		}

		println!( "------------------------------------------" );
	}
}
