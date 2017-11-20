use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a T, y: &'a T, ann: &'a T) -> &'a T
	where T: Display + std::iter::ExactSizeIterator
{
	println!("Announcement! {}", ann);
	if x.len() > y.len() {
		return x
	} else if 1 == 1 {
		return y
	}
	ann
}
