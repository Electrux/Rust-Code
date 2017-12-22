pub mod common;

pub fn infix_to_postfix< 'a >( expression: &'a str,
		     postfix: & mut Vec< String > ) -> Result< (), String >
{
	let mut stack = Vec::new();
	let mut divexpr = common::string_to_string_vec( expression );

	let mut expr: Vec< String > = Vec::new();

	expr.push( "(".to_owned() );
	expr.append( & mut divexpr );
	expr.push( ")".to_owned() );

	for data in expr {
		if common::is_symbol( & data ) {
			if data == "(" {
				stack.push( data );
			}
			else if data == ")" {
				let mut foundmatch = false;

				for i in ( 0 .. stack.len() ).rev() {
					if stack[ i ] == "(" {
						foundmatch = true;
						stack.pop();
						break;
					}

					postfix.push( stack[ i ].clone() );
					stack.pop();
				}

				if !foundmatch {
					return Err( "Invalid expression!".to_owned() );
				}
			}
			else {
				let dataprec = common::get_precedence( & data );
				let mut stackprec =
					common::get_precedence( & stack[ stack.len() - 1 ] );

				if dataprec > stackprec {
				//	stack.push( data );
				}
				else if dataprec == stackprec {

					let res = stack.pop();

					match res {
						Some( val ) => {
							postfix.push( val );
						}
						None => {
							println!( "Unable to pop!" );
						}
					}
				}
				else {
					while dataprec <= stackprec {
						let res = stack.pop();

						match res {
							Some( val ) => {
								postfix.push( val );
							}
							None => {
								println!( "Unable to pop!" );
							}
						}

						if stack.is_empty() {
							break;
						}
						stackprec = common::get_precedence(
							& stack[ stack.len() - 1 ] );
					}
				}

				stack.push( data );
			}
		}
		else {
			postfix.push( data );
		}
	}

	Ok( () )
}
