

fn main()
{
	let mut ctr = 0;

	let mut nums: u64 = 1;

	let mut sum: u64 = 0;

	const MAX: i32 = 1000;

	while ctr < MAX {

		let sqrt = ( nums as f64 ).sqrt() as u64;
		
		if check_prime( & nums, & sqrt ) == true {
			
			println!( "Prime {}: {} {}", ctr, nums, sqrt );
			ctr += 1;
			sum += nums;
		}

		nums += 1;
	}

	println!( "Sum: {}", sum );
}

fn check_prime( num: & u64, sqrt: & u64 ) -> bool
{
	for i in 2 .. *sqrt + 1 {

		if *num % i == 0 {
			return false
		}
	}

	return true
}
