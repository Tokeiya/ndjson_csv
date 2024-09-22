use crate::syntax_node::prelude::*;
use combine::parser::char as chr;
use combine::{Parser, Stream};

pub fn boolean<I: Stream<Token = char>>() -> impl Parser<I, Output = TerminalNode> {
	chr::string::<I>("true")
		.or(chr::string::<I>("false"))
		.map(|str| {
			if str == "true" {
				TerminalNode::new(TerminalNodeType::Boolean, "true".to_string())
			} else if str == "false" {
				TerminalNode::new(TerminalNodeType::Boolean, "false".to_string())
			} else {
				unreachable!()
			}
		})
}

#[cfg(test)]
mod test {

	use super::*;
	#[test]
	fn boolean() {
		let mut parser = super::boolean::<&str>();
		let (a, rem) = parser.parse("true").unwrap();
		assert_eq!(rem, "");
		a.assert(TerminalNodeType::Boolean, "true");

		let (a, rem) = parser.parse("false").unwrap();
		assert_eq!(rem, "");
		a.assert(TerminalNodeType::Boolean, "false");
	}

	#[test]
	fn invalid() {
		let mut parser = super::boolean::<&str>();
		assert!(parser.parse("True").is_err());
		assert!(parser.parse("False").is_err())
	}
}
