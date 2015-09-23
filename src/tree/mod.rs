use std::cmp::Ordering;
use std::boxed::Box;
use std::vec::Vec;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct HuffmanTree {
	pub zero : Option<Box<HuffmanTree>>,
	pub one : Option<Box<HuffmanTree>>,
	pub count : i32,
	pub elem : Vec<u8>
}

impl HuffmanTree {
	fn empty() -> HuffmanTree {
		HuffmanTree {
			zero : None,
			one : None,
			count : 0,
			elem : Vec::new()
		}
	}
}

impl PartialOrd for HuffmanTree {
	fn partial_cmp(&self, other: &HuffmanTree) -> Option<Ordering> {
        Some(other.count.cmp(&self.count))
    }
}


impl Ord for HuffmanTree {
	fn cmp(&self, other: &HuffmanTree) -> Ordering {
        other.count.cmp(&self.count)
    }
}

pub fn build_tree(data : &Vec<u8>) -> HuffmanTree {
	if data.len() == 1 {
		return HuffmanTree {
			one : None,
			zero : Some(Box::new(HuffmanTree{
				one : None,
				zero : None,
				count : 1,
				elem : vec![data[0]]
			})),
			count : 1,
			elem : vec![data[0]]
		}
	}

	let mut all_nodes : Vec<_> = (0..255).map(|n| {
		let mut elems = Vec::new();
		elems.push(n);
		HuffmanTree {
			zero : None,
			one : None,
			count : 0,
			elem : elems
			}
		}).collect();

	//gather statistics
	for byte in data {
		let index = byte.clone() as usize;
		all_nodes[index].count += 1;
	}

	//remove nodes that don't count
	all_nodes = all_nodes.into_iter().filter(|i| i.count > 0).collect::<Vec<_>>();

	while all_nodes.len() > 1 {
		all_nodes.sort_by(|n1, n2| n2.count.cmp(&n1.count));

		let pop1 = all_nodes.pop();
		let pop2 = all_nodes.pop();

		match (pop1, pop2) {
			(Some(n1), Some(n2)) => {
				let mut new_elems = Vec::new();
				
				for e1 in n1.clone().elem {new_elems.push(e1);}
				for e2 in n2.clone().elem {new_elems.push(e2);}

				let sum = n1.count + n2.count;

				let new_node = HuffmanTree {
					zero : Some(Box::new(n1)),
					one : Some(Box::new(n2)),
					count : sum,
					elem : new_elems
				};
				all_nodes.insert(0, new_node);
			},
			_ => break,
		}
	}

	match all_nodes.pop() {
		Some(node) => node,
		_ => HuffmanTree::empty(),
	}	
}

mod hufftests {
	use super::HuffmanTree;
	use super::build_tree;

	#[test]
	fn smoke_test1() {
		let mut vec : Vec<u8> = Vec::new();

		vec.push(0);
		let vec2 = vec.clone();

		let mut bitty = Vec::new();
		bitty.push(0);

		let tree = HuffmanTree {
			zero : Some(Box::new(HuffmanTree {
				zero : None,
				one : None,
				count : 1,
				elem : bitty.clone()
			})),
			one : None,
			count : 1,
			elem : bitty
		};
		assert_eq!(tree, build_tree(&vec2));
	}

	#[test]
	fn smoke_test2() {
		let mut vec : Vec<u8>= Vec::new();

		vec.push(1);
		let vec2 = vec.clone();

		let mut bitty = Vec::new();
		bitty.push(1);

		let tree = HuffmanTree {
			zero : Some(Box::new(HuffmanTree {
				zero : None,
				one : None,
				count : 1,
				elem : bitty.clone()
			})),
			one : None,
			count : 1,
			elem : bitty
		};

		assert_eq!(tree, build_tree(&vec2));
	}

	#[test]
	fn smoke_test3() {
		let mut vec : Vec<u8>= Vec::new();
		let vec2 = vec.clone();

		let tree = HuffmanTree::empty();

		assert_eq!(tree, build_tree(&vec2));
	}

	#[test]
	fn smoke_code_2_elems() {
		let mut stream : Vec<u8>= vec![2, 2, 0];

		let node_for_2 = HuffmanTree {
			zero : None,
			one : None,
			count : 2,
			elem : vec![2]
		};

		let node_for_0 = HuffmanTree {
			zero : None,
			one : None,
			count : 1,
			elem : vec![0]
		};

		let tree = HuffmanTree {
			zero : Some(Box::new(node_for_0)),
			one : Some(Box::new(node_for_2)),
			count : 3,
			elem : vec![0, 2]
		};

		assert_eq!(tree, build_tree(&stream));
	}

	#[test]
	fn test_tree_has_elems_1() {
		let mut stream : Vec<u8>= vec![1,2,3,4,5,6,7,8,9,10];

		assert_eq!(10, build_tree(&stream).elem.len());
	}

	#[test]
	fn test_tree_has_elems_2() {
		let mut stream : Vec<u8>= vec![1,1,2,2,2,2,0,1,0];
		let tree = build_tree(&stream);

		assert_eq!(3, tree.elem.len());

		println!("Tree: {:?}", tree);
	}
}