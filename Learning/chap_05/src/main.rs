fn main()
{
	let rect = Rectangle{ len: 5, wid: 5 };

	println!( "Area: {}", rect.area() );
}

struct Rectangle
{
	len: usize,
	wid: usize,
}

impl Rectangle
{
	fn area( & self ) -> usize
	{
		self.len * self.wid
	}
}
