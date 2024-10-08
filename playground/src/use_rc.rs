use std::cell::RefCell;
use std::rc::{Rc, Weak};
#[allow(dead_code)]
pub enum Value {
	Terminal(i32),
	NonTerminal(Vec<Rc<Node>>),
}
#[allow(dead_code)]
pub struct Node {
	value: Value,
	parent: RefCell<Weak<Node>>,
}

#[allow(dead_code)]
impl Node {
	pub fn new_terminal(value: i32) -> Rc<Self> {
		Rc::new(Self {
			value: Value::Terminal(value),
			parent: RefCell::new(Weak::default()),
		})
	}

	pub fn new_nonterminal(children: impl IntoIterator<Item = Rc<Node>>) -> Rc<Self> {
		let ret = Rc::new(Node {
			value: Value::NonTerminal(children.into_iter().collect()),
			parent: RefCell::new(Weak::default()),
		});

		let Value::NonTerminal(v) = &ret.value else {
			unreachable!()
		};

		for elem in v.iter() {
			elem.set_parent(ret.clone());
		}

		ret
	}

	fn set_parent(&self, parent: Rc<Node>) {
		let parent = Rc::downgrade(&parent);
		self.parent.replace(parent);
	}

	pub fn parent(&self) -> Option<Rc<Node>> {
		self.parent.borrow().upgrade()
	}

	pub fn value(&self) -> &Value {
		&self.value
	}
}

#[allow(dead_code)]
pub fn usage() {
	let children = vec![Node::new_terminal(42), Node::new_terminal(43)];
	let _ = Node::new_nonterminal(children);
}
