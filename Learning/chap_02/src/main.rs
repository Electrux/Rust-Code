extern crate rand;

use rand::Rng;

fn main()
{
	println!( "Guess the number! : " );

	let secret_no = rand::thread_rng().gen_range( 1, 101 );

	loop {
		let mut guess = String::new();

		std::io::stdin().read_line( &mut guess )
			.expect( "Failed to read line!" );

		println!( "You guessed: {}", guess );
		println!( "The secret number is: {}", secret_no );

		let guess: u32 = match guess.trim().parse() {
			Ok( num ) => num,
			Err( _ ) => {
				println!( "Incorrect data type!" );
				continue;
			},
		};

		match guess.cmp( &secret_no ) {

			std::cmp::Ordering::Equal   => {
				println!( "You are right! Awesome!!" );
				break;
			},
			std::cmp::Ordering::Less    => println!( "Your guess falls down to dust!" ),
			std::cmp::Ordering::Greater => println!( "Your guess sky rockets to space!" ),
		}
	}
}
