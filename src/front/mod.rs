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
					//write_file(compressed.0, compressed.1);
					let thing = compressed.1.serialize();
					println!("This is the {}-byte serial: {:?}", thing.len(), thing);
				},
				Err(_) => println!("Ugh. Couldn't do it."),
			}
		},
		Err(_) => {
			println!("Failed to open destination file at {:?}", path);
		},
	}
}

fn write_file(data : Vec<u8>, tree : HuffmanTree) {
	let header = tree.serialize();
	let offset = header.len() / 2;

	println!("Header is {} bytes", header.len());
}