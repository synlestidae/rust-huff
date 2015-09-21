use tree::{build_tree, HuffmanTree};

pub fn compress_data(data : &Vec<u8>) -> (Vec<u8>, HuffmanTree) {
	let tree = build_tree(data);

	let mut all_bits : Vec<bool> = Vec :: new();

	for byte in data {
		all_bits.extend(&walk_to_byte(&byte, &tree));
	}

	let mut all_bytes : Vec<u8> = Vec::new();

	let mut byte : u8 = 0;
	let mut count = 0;

	for bit in all_bits {
		if count == 8 {
			all_bytes.push(byte);
			byte = 0;
			count = 0;
		}
		if bit {
			byte = (byte << 1) + 1;
		}else{
			byte = (byte << 1);
		}
		count += 1;
	}

	if (count > 0) {
		all_bytes.push(byte << (8 - count)) ;
	}

	return (all_bytes, tree);
}

pub fn decompress_data(data : & Vec<u8>, tree : &HuffmanTree) -> Vec<u8>{
	let mut result = Vec::new();
	let mut all_bits : Vec<bool> = Vec::new();

	for byte in data {
		for bit in make_bits(byte.clone()) {
			all_bits.push(bit);
		}
	}

	while all_bits.len() > 0 { 
		result.push(decompress_codeword(&mut all_bits, tree));
	}

	return result;
}

fn make_bits(byte : u8) -> Vec<bool> {
	let bits = vec![byte & 1, (byte >> 1) & 1,(byte >> 2) & 1, (byte >> 3) & 1, (byte >> 4) & 1, (byte >> 5) & 1,(byte >> 6 & 1),(byte >> 7) & 1];
	let mut bool_bits = Vec::new();

	for bit in bits {
		bool_bits.push((bit == 1));
	}

	return bool_bits;
}

fn decompress_codeword(bits : &mut Vec<bool>, tree : &HuffmanTree) -> u8 {
	if bits.len() > 0 {
		let bit = bits.remove(0);
		if bit {
			match tree.one {
				Some(ref subtree) => return decompress_codeword(bits, &*subtree),
				_ => return tree.elem[0]
			}
		}else{
			match tree.zero {
				Some(ref subtree) => return decompress_codeword(bits, &*subtree),
				_ => return tree.elem[0]
			}
		}
	}

	panic!("Cannot decompress when no input given");
}

fn walk_to_byte(byte : &u8, tree : &HuffmanTree) -> Vec<bool> {
	let mut bits;

	match (*tree).zero {
		Some(ref subtree) => {
			if (subtree).elem.contains(byte) {
				bits = walk_to_byte(byte, &*subtree);
				bits.push(false);
				return bits;
			}
		},
		_ => {}
	};

	match (*tree).one {
		Some(ref subtree) => {
			if (subtree).elem.contains(byte) {
				bits = walk_to_byte(byte, &*subtree);
				bits.push(true);
				return bits;
			} 
		},
		_ => {}
	};

	bits = Vec::new();
	return bits;
}

mod hufftests {
	use super::compress_data;
	use super::decompress_data;
	use super::walk_to_byte;
	use tree::{build_tree, HuffmanTree};

	#[test]
	fn simple_tree_test_1 () {
		let tree = build_tree(&vec![0,0,0,0,0,1,0,0,0,2,0,0]);
		let x : u8 = 0;
		let y : u8 = 1;

		assert_eq!(1, walk_to_byte(&x, &tree).len());
		assert!((walk_to_byte(&x, &tree).len() < walk_to_byte(&y, &tree).len()));
	}

	#[test]
	fn simple_tree_test_2 () {
		let tree = build_tree(&vec![0,0,3,3,3,0,0,1,0,0,3,0,1,0,4,0,0,0,0]);
		let most_frequent : u8 = 0;
		let more_frequent : u8 = 3;
		let frequent : u8 = 1;
		let least_frequent = 4;

		assert_eq!(1, walk_to_byte(&most_frequent, &tree).len());

		assert!((walk_to_byte(&most_frequent, &tree).len() < walk_to_byte(&more_frequent, &tree).len()));
		assert!((walk_to_byte(&more_frequent, &tree).len() <= walk_to_byte(&frequent, &tree).len()));
		assert!((walk_to_byte(&frequent, &tree).len() <= walk_to_byte(&least_frequent, &tree).len()));
	}

	#[test]
	fn simple_tree_walk() {
		let first_left_child = HuffmanTree {
			zero : None,
			one : None,
			count : 1,
			elem : vec![0]
		};

		let bottom_left_child = HuffmanTree {
			zero : None,
			one : None,
			count : 1,
			elem : vec![1]
		};

		let bottom_right_child = HuffmanTree {
			zero : None,
			one : None,
			count : 1,
			elem : vec![2]
		};

		let internal_node = HuffmanTree {
			zero : Some(Box::new(bottom_left_child)),
			one : Some(Box::new(bottom_right_child)),
			count : 2,
			elem : vec![1,2]
		};

		let tree = HuffmanTree {
			zero : Some(Box::new(first_left_child)),
			one : Some(Box::new(internal_node)),
			count : 3,
			elem : vec![0,1,2]
		};

		assert_eq!(vec![false], walk_to_byte(&0, &tree));
		assert_eq!(vec![false, true], walk_to_byte(&1, &tree));
		assert_eq!(vec![true, true], walk_to_byte(&2, &tree));
	}

	#[test]
	fn simple_tree_1() {
		let mut original_data = vec![0,0,0,0,0,0,0,0,0,1];
		let mut compressed = compress_data(&original_data);

		assert_eq!(vec![1,0], compressed.1.elem);

		assert_eq!(Some(Box::new(HuffmanTree{
			zero : None,
			one : None,
			count : 9,
			elem : vec![0]
		})), compressed.1.one);

		assert_eq!(Some(Box::new(HuffmanTree{
			zero : None,
			one : None,
			count : 1,
			elem : vec![1]
		})), compressed.1.zero);
	}

	#[test]
	fn simple_compress_decompress_test_1() {
		let mut original_data = vec![0];
		let mut compressed = compress_data(&original_data);

		let decompressed = decompress_data(&compressed.0, &compressed.1);

		assert_eq!(1, compressed.0.len());
		assert_eq!(original_data, decompressed); 
		assert_eq!(1, decompressed.len());
	}
}