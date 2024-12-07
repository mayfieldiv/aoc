fn _main() -> anyhow::Result<()> {
	let mut input = String::new();
	std::io::Read::read_to_string(&mut std::io::stdin(), &mut input)?;

	#[derive(Debug)]
	enum Step {
		Start,
		M,
		U,
		L,
		OpenParen,
		Number1(i32),
		Number2(i32, i32),
		Done(i32, i32),
	}

	let mut result = 0;
	let mut step = Step::Start;
	for c in input.chars() {
		step = match step {
			Step::Start if c == 'm' => Step::M,
			Step::M if c == 'u' => Step::U,
			Step::U if c == 'l' => Step::L,
			Step::L if c == '(' => Step::OpenParen,
			Step::OpenParen if c.is_digit(10) => Step::Number1(c.to_string().parse::<i32>().unwrap()),
			Step::Number1(n) if c.is_digit(10) => Step::Number1(10 * n + c.to_string().parse::<i32>().unwrap()),
			Step::Number1(n) if c == ',' => Step::Number2(n, 0),
			Step::Number2(n, m) if c.is_digit(10) => Step::Number2(n, 10 * m + c.to_string().parse::<i32>().unwrap()),
			Step::Number2(n, m) if c == ')' => Step::Done(n, m),
			_ => Step::Start,
		};
		// println!("{:?}", step);
		if let Step::Done(n, m) = step {
			result += n * m;
			step = Step::Start;
		}
	}

	println!("{}", result);
	Ok(())
}

fn main() -> anyhow::Result<()> {
	let mut input = String::new();
	std::io::Read::read_to_string(&mut std::io::stdin(), &mut input)?;

	#[derive(Debug)]
	enum Step {
		Start,
		M,
		Mu,
		Mul,
		MulOpenParen,
		MulNumber1(i32),
		MulNumber2(i32, i32),
		MulDone(i32, i32),
		D,
		Do,
		DoOpenParen,
		DoDone,
		Don,
		DonApostrophe,
		Dont,
		DontOpenParen,
		DontDone,
	}

	let mut result = 0;
	let mut mul_enabled = true;
	let mut step = Step::Start;
	for c in input.chars() {
		step = match step {
			Step::Start if c == 'm' => Step::M,
			Step::M if c == 'u' => Step::Mu,
			Step::Mu if c == 'l' => Step::Mul,
			Step::Mul if c == '(' => Step::MulOpenParen,
			Step::MulOpenParen if c.is_digit(10) => Step::MulNumber1(c.to_string().parse::<i32>().unwrap()),
			Step::MulNumber1(n) if c.is_digit(10) => Step::MulNumber1(10 * n + c.to_string().parse::<i32>().unwrap()),
			Step::MulNumber1(n) if c == ',' => Step::MulNumber2(n, 0),
			Step::MulNumber2(n, m) if c.is_digit(10) => Step::MulNumber2(n, 10 * m + c.to_string().parse::<i32>().unwrap()),
			Step::MulNumber2(n, m) if c == ')' => Step::MulDone(n, m),

			Step::Start if c == 'd' => Step::D,
			Step::D if c == 'o' => Step::Do,
			Step::Do if c == '(' => Step::DoOpenParen,
			Step::DoOpenParen if c == ')' => Step::DoDone,

			Step::Do if c == 'n' => Step::Don,
			Step::Don if c == '\'' => Step::DonApostrophe,
			Step::DonApostrophe if c == 't' => Step::Dont,
			Step::Dont if c == '(' => Step::DontOpenParen,
			Step::DontOpenParen if c == ')' => Step::DontDone,

			_ => Step::Start,
		};

		// println!("{} {:?}", c, step);
		step = match step {
			Step::MulDone(n, m) => {
				if mul_enabled {
					result += n * m;
				}
				Step::Start
			}
			Step::DoDone => {
				mul_enabled = true;
				Step::Start
			}
			Step::DontDone => {
				mul_enabled = false;
				Step::Start
			}
			_ => step,
		};
	}

	println!("{}", result);
	Ok(())
}
