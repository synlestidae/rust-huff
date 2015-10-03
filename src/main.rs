use std::env;

mod tree;
mod algorithm;
mod front;

fn main() {
	let mut args : Vec<_> = env::args().collect();
	let usage = format!("Usage: input_file");

	if args.len() == 2 {
		println!("Compressing... {}", args[1]);
		front::compress_file(args[1].clone());
	}else if args.len() == 3 {
		println!("Decompressing {}...", args[1]);
		front::decompress_file(args[1].clone(), args[2].clone());

	}else{
		println!("{}", usage);
	}	
}
