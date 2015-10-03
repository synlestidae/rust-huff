use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use tree::{build_tree, HuffmanTree};
use algorithm::{compress_data, decompress_data};

pub fn compress_file(inputfile : String) {
	let path = Path::new(&inputfile);

	let output_file = inputfile.clone() + ".huffed";

	match File::open(&path) {
		Ok(ref mut file) => {
			let mut bytes : &mut Vec<u8> = &mut Vec::new();
			match file.read_to_end(bytes) {
				Ok(count) => {
					println!("Read  {} bytes from {}", count, inputfile);
					let compressed = compress_data(&bytes);

					let compressed_data = compressed.0;
					let mut tree_data = compressed.1.serialize();

					tree_data.extend(compressed_data);

					File::open(&output_file).unwrap().write_all(&*tree_data);

					println!("Done it! Wrote to the output file");
				},
				Err(_) => println!("Ugh. Couldn't do it."),
			}
		},
		Err(_) => {
			println!("Failed to open destination file at {:?}", path);
		},
	}
}

pub fn decompress_file(inputfile : String) {
	let mut data_file = File::open(inputfile).unwrap();
	let mut file_data = Vec::new(); 
	data_file.read_to_end(&mut file_data);

	let mut compressed_data = Vec::new();
	let mut tree_data = Vec::new();

	for i in (0 as usize .. 256) {
		tree_data.push(file_data[i]);
	}

	let original_length = file_data[256] as usize + 
				((file_data[256 + 1] as usize) << 8) + 
				((file_data[256 + 2] as usize) << 16) + 
				((file_data[256 + 3] as usize) << 24); 

	for i in (260 as usize .. file_data.len()) {
		compressed_data.push(file_data[i]);
	}

	let tree = HuffmanTree::deserialize(tree_data);


	let original_data = decompress_data(&compressed_data, &tree, original_length);
}

fn write_file(data : Vec<u8>, tree : HuffmanTree) {
	let header = tree.serialize();
	let offset = header.len() / 2;

	println!("Header is {} bytes", header.len());
}