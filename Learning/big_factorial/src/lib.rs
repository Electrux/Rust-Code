fn vec_to_string( vec: Vec< usize > ) -> String
{
	let mut string = String::new();

	for elem in vec.iter().rev() {
		string += & elem.to_string();
	}

	string
}

pub fn get_factorial( num: usize ) -> String
{
	let mut vec = vec![ 1 ];

	for i in 2 .. num + 1 {

		multiply_vec( & mut vec, i );
	}
	
	vec_to_string( vec )
}

fn multiply_vec( vec: & mut Vec< usize >, num: usize )
{
	let mut mult: usize = 0;
	
	for i in 0 .. vec.len() {
		mult = vec[ i ] * num + mult;
		vec[ i ] = mult % 10;

		mult /= 10;
	}

	while mult > 0 {
		vec.push( mult % 10 );

		mult /= 10;
	}
}
