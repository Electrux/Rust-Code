pub mod intopost;

use self::intopost::common;

pub fn eval_expr( expression: & str ) -> Result< i64, String >
{
	if expression.len() <= 0 {
		return Err( "Nothing to evaluate!".to_owned() );
	}

	let mut postfix = Vec::new();

	intopost::infix_to_postfix( expression, & mut postfix )?;

	println!( "{:?}", postfix );

	if postfix.is_empty() {
		return Err( "No value entered!".to_owned() );
	}

	let ans = eval_postfix( & mut postfix )?;

	Ok( ans )
}

fn eval_postfix( postfix: & mut Vec< String > ) -> Result< i64, String >
{
	if postfix.len() < 3 {
		let errmsg =
			format!( "How do u expect to calculate something\n
				  without at least 3 values?" );

		println!( "{}", errmsg );

		return Err( errmsg )
	}

	let mut stack: Vec< String > = Vec::new();

	for elem in postfix {
		if common::is_symbol( elem ) {
			if stack.len() < 2 {
				let errmsg =
					format!( "Invalid expression!" );

				println!( "{}", errmsg );

				return Err( errmsg )
			}

			let mut a: i64 = 0;
			let mut b: i64 = 0;

			let res1 = stack.pop();

			match res1 {
				Some( val ) => {
					a = common::fetch_integer( & val )?;
				}
				None => {}
			}

			let res2 = stack.pop();

			match res2 {
				Some( val ) => {
					b = common::fetch_integer( & val )?;
				}
				None => {}
			}

			stack.push( calc_values( a, b, elem ) );
		}
		else {
			stack.push( elem.clone() );
		}
	}

	let ans = common::fetch_integer( & stack[ 0 ] )?;

	Ok( ans )
}

fn calc_values( a: i64, b: i64, op: & str ) -> String
{
	let mut ans: i64 = 0;

	if op == "+" {
		ans = b + a;
	}
	else if op == "-" {
		ans = b - a;
	}
	else if op == "*" {
		ans = b * a;
	}
	else if op == "/" {
		ans = b / a;
	}
	else if op == "%" {
		ans = b % a;
	}
	else if op == "^" {
		ans = b.pow( a as u32 );
	}

	ans.to_string()
}
