use std::io::Write;

pub mod employees;

use self::employees::*;

fn main()
{
	let mut employees = Employees::new();

	loop {
		println!( "What do you want to do?" );
		println!( "1 : Insert new employee" );
		println!( "2 : Delete existing employee" );
		println!( "3 : Show all employees" );
		println!( "4 : Quit" );
		print!( "Enter your choice : " );
		std::io::stdout().flush()
			.expect( "IO not working correctly" );

		let mut line = String::new();

		std::io::stdin().read_line( & mut line )
			.expect( "Unable to read line!" );

		let choice: u64 = line.trim().parse()
			.expect( "Expected a number!" );

		match choice {

			1 => unsafe {

				employees.create_new_employee()
					.expect( "Unable to create employee" );
			}

			2 => unsafe {
				employees.delete_employee()
					.expect( "Unable to delete employee" );
			}

			3 => {
				employees.display_all();
			}

			_ => {
				break;
			}
		}
	}

	println!( "Exiting..." );
}
