#![allow(non_snake_case)]
use chrono::{TimeZone, Utc};
use users::get_user_by_uid;
use std::{fs, env};
use std::os::unix::fs::{PermissionsExt, MetadataExt};
use std::time::UNIX_EPOCH;

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

	print_filenames(&file_vec, width_rec);
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

fn print_long(v: &Vec<String>, path: &str) {
	for filename in v {
		let file_path = &format!("{}/{}", path, filename);

		if let Ok(metadata) = fs::symlink_metadata(file_path) {
			if metadata.is_dir() {
				print!("d"); 
			} else if metadata.file_type().is_symlink() {
				print!("l"); 
			} else {
				print!("-"); 
			}

			print_permissions(metadata.permissions().mode());

			print!(" {}", metadata.nlink());
			print!(" {}", get_user_by_uid(metadata.uid()).unwrap().name().to_str().unwrap());
			print!(" {}", get_user_by_uid(metadata.uid()).unwrap().groups()
				.unwrap()[0].name().to_str().unwrap());
			print!(" {:-1$}", metadata.len(), 4);
			if let Ok(modtime) = metadata.modified() {
				let seconds = modtime.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
				let dt = Utc.timestamp(seconds, 0);
				print!(" {}", dt.format("%b"));
				print!(" {}", dt.format("%d %H:%M"));
			} else {
				panic!("Error getting modification time.");
			}
			print!(" {}", filename);

			if let Ok(link_to) = fs::read_link(file_path) {
				print!(" -> {}", link_to.display());
			}
			print!("\n");
		} else {
			panic!("Error getting file metadata.");
		}
	}
}

fn get_bit(mode: u32, index: u8) -> bool {
//	println!("mode is: {}\n1 << {} is: {}", mode, index, 1 << index);

	if index < 32 {
		mode & (1 << index) != 0
	} else {
		panic!("Bit index out of bounds.");
	}
}

fn print_permissions(mode: u32) {
	if get_bit(mode, 8) { print!("r"); } else { print!("-"); }
	if get_bit(mode, 7) { print!("w"); } else { print!("-"); }
	if get_bit(mode, 6) { print!("x"); } else { print!("-"); }
	if get_bit(mode, 5) { print!("r"); } else { print!("-"); }
	if get_bit(mode, 4) { print!("w"); } else { print!("-"); }
	if get_bit(mode, 3) { print!("x"); } else { print!("-"); }
	if get_bit(mode, 2) { print!("r"); } else { print!("-"); }
	if get_bit(mode, 1) { print!("w"); } else { print!("-"); }
	if get_bit(mode, 0) { print!("x"); } else { print!("-"); }
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

	v.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

	if flags.R_flag {
		print_recursive(&path, width, &flags);
	} else if flags.l_flag {
		print_long(&v, &path);
	} else {
		print_filenames(&v, width);
		print!("\n");
	}
}
