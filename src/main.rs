use std::{fs, env};

pub fn longest_len(v: &Vec<String>) -> usize {
	let mut result = v[0].len();

	for element in v.iter() {
		if element.len() > result {
			result = element.len();
		}
	}

	result
}

pub fn print_filenames(v: &Vec<String>, width: usize) {
	let mut i = 0;

	for filename in v {
		if i != 0 && i % 6 == 0 {print!("\n");}

		print!("{:-1$}", filename, width);

		i += 1;
	}
}

fn main() {
	let mut args = env::args();
	args.next();

	let mut v = vec![];

	let path = match args.next() {
		Some(arg) => arg,
		None => String::from("."),
	};

	if let Ok(entries) = fs::read_dir(path) {

		for entry in entries {
			if let Ok(entry) = entry {
				v.push(entry.file_name().into_string().unwrap());
			} else {
				print!("Error.")
			}
		}
	}

	let width = longest_len(&v) + 1;

	v.sort_unstable();

	print_filenames(&v, width);

	print!("\n");
}
