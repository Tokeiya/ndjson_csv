use super::node::Node;
use crate::syntax_node::node_value::NodeValue;
use std::collections::vec_deque::VecDeque;
use std::rc::Rc;

pub struct BreadthFirstIterator(VecDeque<Rc<Node>>);

impl BreadthFirstIterator {
	pub fn new(root: Rc<Node>) -> BreadthFirstIterator {
		let mut vec = VecDeque::new();
		vec.push_back(root);
		BreadthFirstIterator(vec)
	}
}
impl Iterator for BreadthFirstIterator {
	type Item = Rc<Node>;

	fn next(&mut self) -> Option<Self::Item> {
		let piv = self.0.pop_front()?;

		match piv.value() {
			NodeValue::Terminal(_) => Some(piv),
			NodeValue::Array(arr) => {
				for elem in arr.value().iter() {
					self.0.push_back(elem.clone());
				}

				Some(piv)
			}
			NodeValue::Object(obj) => {
				for elem in obj.value().iter() {
					self.0.push_back(elem.clone());
				}
				Some(piv)
			}
		}
	}
}

#[cfg(test)]
mod test {
	use crate::syntax_node::test_prelude::*;

	use super::*;
	use std::borrow::Borrow;

	#[test]
	fn new() {
		let root = node_helper::gen_sample();
		let fixture = BreadthFirstIterator::new(root.clone());
		assert!(std::ptr::eq(
			root.borrow() as *const Node,
			fixture.0[0].borrow() as *const Node
		))
	}

	#[test]
	fn breadth_first_iterator() {
		let expected = vec![
			"Root",
			"Root::arr",
			"Root::arr::[0]",
			"Root::arr::[1]",
			"Root::arr::[0]0_0",
			"Root::arr::[0]1_1",
			"Root::arr::[1][0]",
			"Root::arr::[1][1]",
			"Root::arr::[1][2]",
			"Root::arr::[1][3]",
			"Root::arr::[1][2][0]",
			"Root::arr::[1][2][1]",
			"Root::arr::[1][3]1_0",
			"Root::arr::[1][3]1_1",
		];

		let root = node_helper::gen_sample();
		let ite = BreadthFirstIterator::new(root.clone());

		let mut cnt = 0usize;

		for actual in ite {
			assert_eq!(
				actual.full_qualified_name().text_expression(),
				expected[cnt]
			);
			cnt += 1;
		}

		assert_eq!(expected.len(), cnt);
	}
}
