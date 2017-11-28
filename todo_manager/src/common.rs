
pub fn combineparams( args: & Vec< String >, from: usize, to: usize ) -> String
{
	if from >= to {
		return String::new()
	}

	let mut combined: String = args[ from ].clone();

	for i in from + 1 .. to {
		combined = combined + " ";
		combined = combined + & args[ i ];
	}

	combined
}
