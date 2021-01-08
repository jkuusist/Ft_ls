#![allow(non_snake_case)]
use std::{fs, env};

struct Flags {
	l_flag: bool,
	R_flag: bool,
	a_flag: bool,
	r_flag: bool,
	t_flag: bool,
}

impl Flags {
	fn new() -> Flags {
		Flags {
			l_flag: false,
			R_flag: false,
			a_flag: false,
			r_flag: false,
			t_flag: false,
		}
	}

	fn toggle_on(&mut self, flag_name: char) {
		match flag_name {
			'l' => { self.l_flag = true; }
			'R' => { self.R_flag = true; }
			'a' => { self.a_flag = true; }
			'r' => { self.r_flag = true; }
			't' => { self.t_flag = true; }
			_ => {panic!("Invalid option");}
		}
	}
}

fn longest_len(v: &Vec<String>) -> usize {
	if v.is_empty() {
		0
	} else {
		let mut result = v[0].len();

		for element in v.iter() {
			if element.len() > result {
				result = element.len();
			}
		}
		result
	}
}

fn print_filenames(v: &Vec<String>, width: usize) {
	let mut i = 0;

	if v.len() < 13 {
		for filename in v {
			print!("{}  ", filename);
		}
	} else {
		for filename in v {
			if i != 0 && i % 6 == 0 {print!("\n");}

			print!("{:-1$}", filename, width);

			i += 1;
		}
	}
}

fn print_recursive(path: &str, width: usize, flags: &Flags) {
	print!("{}:\n", path);

	let mut file_vec = vec![];

	for entry in fs::read_dir(path).unwrap() {
		let current = entry.unwrap().file_name().into_string().unwrap();

		if current.starts_with('.') && !flags.a_flag {
			continue;
		} else {
			file_vec.push(current);
		}
	}

	file_vec.sort_unstable();

	let width_rec = longest_len(&file_vec);

//	for file in file_vec {
//		print!("{} ", file);
		print_filenames(&file_vec, width_rec);
//	}
	print!("\n");

	for entry in fs::read_dir(path).unwrap() {
		let path_buf = entry.unwrap().path();
		let file_path = path_buf.display().to_string();
		let attrib = fs::metadata(&file_path).unwrap();

		if !flags.a_flag && file_path.starts_with("./.") {
			continue;
		}

		if attrib.is_dir() {
			print!("\n");
			print_recursive(&file_path, width, flags);
		}
	}
}

fn main() {
	let mut path = String::from(".");
	let mut flags = Flags::new();
	let mut args = env::args();
	args.next();

	for arg in &mut args {
		if arg.starts_with('-') {
			for char in arg.chars() {
				if char != '-' {
					flags.toggle_on(char);
				}
			}
		} else {
			path = arg;
		}
	}

	let mut v = vec![];

	if flags.a_flag {
		v.push(String::from("."));
		v.push(String::from(".."));
	}

	if let Ok(entries) = fs::read_dir(&path) {
		for entry in entries {
			if let Ok(entry) = entry {
				if entry.file_name().into_string().unwrap().starts_with('.') && 
				!flags.a_flag {
					continue;
				} else {
					v.push(entry.file_name().into_string().unwrap());
				}
			} else {
				print!("Error.")
			}
		}
	}

	let width = longest_len(&v) + 1;

	v.sort_unstable();

	if flags.R_flag {
		print_recursive(&path, width, &flags);
	} else {
		print_filenames(&v, width);
		print!("\n");
	}
}
