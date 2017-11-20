pub struct Employee
{
	id: u64,
	name: String,
	title: String,
}

impl Employee
{
	pub fn new( name: & str, title: & str, emp_count: u64 ) -> Employee
	{
		let mut emp = Employee{ id: 0, name: String::new(), title: String::new() };

		emp.id = emp_count;
		emp.name = name.to_string();
		emp.title = title.to_string();

		emp
	}

	pub fn display( & self )
	{
		println!( "ID: {}", self.id );
		println!( "Name: {}", self.name );
		println!( "Title: {}", self.title );
	}

	pub fn get_id( & self ) -> & u64 { & self.id }
	pub fn get_name( & self ) -> & String { & self.name }
	pub fn get_title( & self ) -> & String { & self.title }

	pub fn set_name( & mut self, name: String ) { self.name = name }
	pub fn set_title( & mut self, title: String ) { self.title = title }
}
