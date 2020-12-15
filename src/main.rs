use std::{fs, env};

fn main() {
	let mut args = env::args();

	args.next();

	let path = match args.next() {
		Some(arg) => arg,
		None => String::from("."),
	};

	if let Ok(entries) = fs::read_dir(path) {
		for entry in entries {
			if let Ok(entry) = entry {
				print!("{} ", entry.file_name().into_string().unwrap());
			} else {
				print!("Error.")
			}
		}
		print!("\n");
	}
}
