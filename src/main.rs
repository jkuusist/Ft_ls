use std::fs;

fn main() {
	if let Ok(entries) = fs::read_dir(".") {
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
