pub fn run( num: u64 )
{
	let mut num = num;

	if num < 9 {
		eprintln!( "Error: Number entered is less than 9 which won't work!" );
	}
	
	while num > 9 {

		let digits = get_num_digits( num );

		print!( "{}", num );

		for digit in digits {

			print!( " - {}", digit );

			num -= digit;
		}

		println!( " = {}", num );
	}
}


fn get_num_digits( num: u64 ) -> Vec< u64 >
{
	let mut newnum = num;
	let mut res = Vec::new();
	
	while newnum > 0 {

		res.push( newnum % 10 );
		newnum /= 10;
	}

	res
}
