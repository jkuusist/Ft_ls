#![allow(non_snake_case)]
use chrono::{TimeZone, Utc};
use users::get_user_by_uid;
use term_size;
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

fn print_filenames(v: &Vec<String>, output_length: usize, width: usize) {
	let (terminal_width, _) = term_size::dimensions().unwrap();

	if output_length <= terminal_width {
		for filename in v {
			print!("{}  ", filename);
		}
		print!("\n");
	} else {
		let mut num_columns = terminal_width / width;

		let num_rows = v.len() / num_columns;

		if v.len() % 2 != 0 { num_columns += 1 };

		let mut matrix = vec![vec![]; num_columns];

		let mut i = 0;
		let mut j;
		let mut v_index = 0;

		while i < num_columns {
			j = 0;
			while j < num_rows && v_index < v.len() {
				matrix[i].push(&v[v_index]);
				v_index += 1;
				j += 1;
			}
			i += 1;
		}

		i = 0;
		while i < matrix[i].len(){
			let mut j = 0;

			while j < matrix.len() {
				let mut col_width = matrix[j][0].len();

				for row in 0..num_rows {
					if row < matrix[j].len() && matrix[j][row].len() > col_width {
						col_width = matrix[j][row].len();
					}
				}
				if i < matrix[j].len() {
					print!("{:1$}  ", matrix[j][i], col_width);
				}

				j += 1;
			}
			print!("\n");
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
	let mut output_length_rec = 0;

	for filename in &file_vec {
		output_length_rec += filename.len() + 2;
	}

	let width_rec = longest_len(&file_vec);

	if flags.l_flag {
		print_long(&file_vec, path);
	} else {
		print_filenames(&file_vec, output_length_rec, width_rec);
	}
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
				print!(" {}", dt.format("%b %e %H:%M"));
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
	let mut output_length = 0;
	for filename in &v {
		output_length += filename.len() + 2;
	}

	v.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

	if flags.R_flag {
		print_recursive(&path, width, &flags);
	} else if flags.l_flag {
		print_long(&v, &path);
	} else {
		print_filenames(&v, output_length, width);
//		print!("\n");
	}
}
