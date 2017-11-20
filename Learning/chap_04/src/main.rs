fn main()
{
	let mut arr = [ 10, 9, 8, 7, 6, 5, 4, 3, 2, 1 ];

	sort( & mut arr );

	for item in arr.iter() {
		println!( "{}", item );
	}
}

fn sort( mut arr: & mut [ usize ] )
{
	let size = arr.len();

	for i in 0 .. size {

		for j in 0 .. size - i - 1 {

			swap_if_greater_than( & mut arr, j, j + 1 );
		}
	}
}

fn swap_if_greater_than( arr: & mut [ usize ], a: usize, b: usize )
{
	if arr[ a ] > arr[ b ] {
		let t = arr[ a ];
		arr[ a ] = arr[ b ];
		arr[ b ] = t;
	}
}
