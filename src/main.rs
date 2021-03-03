#![allow(non_snake_case)]
use chrono::{TimeZone, Utc};
use users::get_user_by_uid;
use std::{fs, env};
use std::os::unix::fs::{PermissionsExt, MetadataExt};
use std::time::UNIX_EPOCH;
//use std::time::{Duration, Instant};

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

fn longest_len(v: &[String]) -> usize {
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

fn print_filenames(v: &[String], output_length: usize, width: usize) {
	let (terminal_width, _) = term_size::dimensions().unwrap();

	if output_length <= terminal_width {
		for filename in v {
			print!("{}  ", filename);
		}
		println!();
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

		while i < matrix.len() && i < matrix[i].len() {
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
			println!();
			i += 1;
		}
	}
}

fn print_recursive(path: &str, width: usize, flags: &Flags) {
	println!("{}:", path);

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
	println!();

	for entry in fs::read_dir(path).unwrap() {
		let path_buf = entry.unwrap().path();
		let file_path = path_buf.display().to_string();
		let attrib = fs::metadata(&file_path).unwrap();

		if !flags.a_flag && file_path.starts_with("./.") {
			continue;
		}

		if attrib.is_dir() {
			println!();
			print_recursive(&file_path, width, flags);
		}
	}
}

fn print_long(v: &[String], path: &str) {
	let mut output = String::with_capacity(v.len() * 54);
	let mut total_blocks = 0;

	for filename in v {
		let file_path = &format!("{}/{}", path, filename);

		if let Ok(metadata) = fs::symlink_metadata(file_path) {
			total_blocks += metadata.blocks();

			if metadata.is_dir() {
				output.push('d');
			} else if metadata.file_type().is_symlink() {
				output.push('l');
			} else {
				output.push('-');
			}

			push_permissions(metadata.permissions().mode(), &mut output);

			output.push(' ');
			output.push_str(&metadata.nlink().to_string());

			output.push(' ');
			output.push_str(get_user_by_uid(metadata.uid()).unwrap().name().to_str().unwrap());
			output.push(' ');
			output.push_str(get_user_by_uid(metadata.uid()).unwrap().groups()
				.unwrap()[0].name().to_str().unwrap());

			output.push_str(&format!(" {:-1$}", metadata.len(), 4));

			if let Ok(modtime) = metadata.modified() {
				let seconds = modtime.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
				let dt = Utc.timestamp(seconds, 0);
				output.push(' ');
				output.push_str(&dt.format("%b %e %H:%M").to_string());

			} else {
				panic!("Error getting modification time.");
			}

			output.push(' ');
			output.push_str(filename);

			if let Ok(link_to) = fs::read_link(file_path) {
				output.push_str(&format!(" -> {}", link_to.display()));
			}
			output.push('\n');
		} else {
			panic!("Error getting file metadata.");
		}
	}

	println!("total {}", total_blocks / 2);
	print!("{}", output);
}

fn get_bit(mode: u32, index: u8) -> bool {
	if index < 32 {
		mode & (1 << index) != 0
	} else {
		panic!("Bit index out of bounds.");
	}
}

fn push_permissions(mode: u32, output: &mut String) {
	if get_bit(mode, 8) { output.push('r'); } else { output.push('-'); }
	if get_bit(mode, 7) { output.push('w'); } else { output.push('-'); }
	if get_bit(mode, 6) { output.push('x'); } else { output.push('-'); }
	if get_bit(mode, 5) { output.push('r'); } else { output.push('-'); }
	if get_bit(mode, 4) { output.push('w'); } else { output.push('-'); }
	if get_bit(mode, 3) { output.push('x'); } else { output.push('-'); }
	if get_bit(mode, 2) { output.push('r'); } else { output.push('-'); }
	if get_bit(mode, 1) { output.push('w'); } else { output.push('-'); }
	if get_bit(mode, 0) { output.push('x'); } else { output.push('-'); }
}
/*
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
*/
fn sort_by_mod_time(v: &mut Vec<String>, low: usize, high: usize) {
	if low < high {
		let p = partition(v, low, high);
		sort_by_mod_time(v, low, p - 1);
		sort_by_mod_time(v, p + 1, high);
	}
}

fn partition(v: &mut Vec<String>, low: usize, high: usize) -> usize {
	let pivot = fs::symlink_metadata(&v[high]).unwrap().modified().unwrap();

	let mut i = low;
	let mut j = low;
	while j < high {
		let comp = fs::symlink_metadata(&v[j]).unwrap().modified().unwrap();
		if comp > pivot {
			v.swap(i, j);

			i += 1;
		}
		j += 1;
	}
	v.swap(i, high);

	i
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

	if flags.t_flag {
		let v_high = v.len() - 1;
		sort_by_mod_time(&mut v, 0, v_high);
	} else {
//		v.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
		v.sort_unstable_by_key(|a| a.to_lowercase());
	}

	if flags.R_flag {
		print_recursive(&path, width, &flags);
	} else if flags.l_flag {
		print_long(&v, &path);
	} else {
		print_filenames(&v, output_length, width);
//		print!("\n");
	}
}
