use std::env;

mod tree;
mod algorithm;
mod front;

fn main() {
	let mut args = env::args();
	let usage = format!("Usage: input_file");

	match args.nth(1) {
		Some(input_file) => {
			front::compress_file(input_file);
		} 
		_ => println!("{}", usage),
	}
}
