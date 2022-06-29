use alloc::{string::String, vec::Vec};

pub struct Parser {
	pub input: String,
	pub position: usize,
}

impl Parser {
	pub fn new(input: String) -> Self {
		Self { input, position: 0 }
	}

	pub fn parse(&mut self) -> crate::Result<Vec<String>> {
		let mut output = Vec::new();
		while self.position < self.input.len() {
			let token = self.next_token();
			if token.len() > 0 {
				output.push(token);
			}
		}
		Ok(output)
	}

	pub fn next_token(&mut self) -> String {
		let mut token = String::new();
		while self.position < self.input.len() {
			let c = self.input.chars().nth(self.position).unwrap();
			self.position += 1;
			if c == ' ' || c == '\n' || c == '\t' || c == '\r' {
				break;
			}
			token.push(c);
		}
		token
	}
}
