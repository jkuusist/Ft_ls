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
	let mut result = v[0].len();

	for element in v.iter() {
		if element.len() > result {
			result = element.len();
		}
	}

	result
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

fn print_recursive(v: &Vec<String>, width: usize, path: &str, flags: &Flags) {
	print!("{}:\n", path); 
	print_filenames(v, width);
	print!("\n\n");

	if let Ok(entries) = fs::read_dir(&path) {
		for entry in entries {
			if let Ok(entry) = entry {
				if entry.file_name().into_string().unwrap().starts_with('.') 
				&& flags.a_flag == false {
					continue;
				} else {
					let current_path = entry.path();
					if current_path.is_dir() {
							print_recursive(
								v, 
								width, 
								current_path.into_os_string().to_str().unwrap(),
								flags
							);
					}
				}
			}
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
		print_recursive(&v, width, &path, &flags);
	} else {
		print_filenames(&v, width);
	}
	print!("\n");
}
