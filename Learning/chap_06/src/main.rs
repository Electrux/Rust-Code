fn main()
{
	let mut str = std::string::String::new();

	std::io::stdin().read_line( & mut str )
		.expect( "Could not read line for some reason" );

	let mut ctr: usize = 0;

	let str = str.trim();

	for _ in str.chars() {
		ctr += 1;
	}

	println!( "{}", ctr );
}

/*
enum Coin
{
	Penny,
	Nickel,
	Dime,
	Quarter,
}

fn get_in_cents( coin: Coin ) -> u32
{
	match coin {
		Coin::Penny   => 1,
		Coin::Nickel  => 5,
		Coin::Dime    => 10,
		Coin::Quarter => 25,
	}
}

fn main()
{
	let coin = Coin::Nickel;

	if let coin = Coin::Nickel {
		println!( "Yo!" );
	}
}
*/
