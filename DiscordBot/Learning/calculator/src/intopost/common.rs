pub fn string_to_string_vec( text: & str ) -> Vec< String >
{
	let mut strs: Vec< String > = Vec::new();

	let mut tempstr: String = String::new();

	for ch in text.chars() {
		if is_symbol( & ch.to_string() ) {
			if !tempstr.is_empty() {
				strs.push( tempstr.clone() );
				tempstr.clear();
			}
			tempstr.push( ch );
			strs.push( tempstr.clone() );
			tempstr.clear();

			continue;
		}
		else if ch == ' '{
			if !tempstr.is_empty() {
				strs.push( tempstr.clone() );
				tempstr.clear();
			}

			continue;
		}

		tempstr.push( ch );
	}

	if !tempstr.is_empty() {
		strs.push( tempstr.clone() );
	}

	return strs;
}

pub fn is_symbol( data: & str ) -> bool
{
	if data == "(" || data == ")" || data == "^" || data == "/" || data == "*" ||
	    data == "%" || data == "+" || data == "-" {
		return true;
	}

	return false;
}

pub fn get_precedence( symbol: & str ) -> i32
{
	if symbol == "(" || symbol == ")" {
		return -1;
	}
	if symbol == "^" {
		return 3;
	}
	if symbol == "/" || symbol == "*" || symbol == "%" {
		return 2;
	}
	if symbol == "+" || symbol == "-" {
		return 1;
	}
	return 0;
}

pub fn fetch_integer( text: & str ) -> Result< i64, String >
{
	let res = text.trim().parse();

	match res {
		Ok( val ) => {
			return Ok( val )
		},
		Err( err ) => {
			let errmsg =
				format!( "Unable to parse integer {}!", err );

			println!( "{}", errmsg );

			return Err( errmsg )
		},
	}
}
