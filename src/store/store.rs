use std::fs;

use directories::ProjectDirs;

pub trait Store: std::io::Read {
	fn store_local_data(data: &[u8], project_name: &str) -> Result<(), String> where Self: Sized, Self: AsRef<[u8]> {
		if let Some(dir_string) = resolve_path(project_name) {
			if let Err(error) = fs::write(dir_string, data) {
				Err(format!("{}", error))
			} else {
				Ok(())
			}
		} else {
			Err(format!("Cannot resolve {} to path", project_name))
		}
	}
	fn load_local_data(project_name: &str) -> Result<Vec<u8>, String> where Self: Sized {
		if let Some(dir_string) = resolve_path(project_name) {
			if let Ok(from_reader) = fs::read(dir_string) {
				return Ok(from_reader);
			} else {
				Err(format!("Failed to read file"))
			}
		} else {
			Err(format!("Cannot resolve {} to path", project_name))
		}
	}
}

fn resolve_path(project_name: &str) -> Option<String> {
	if let Some(dirs) = ProjectDirs::from("", "", project_name) {
		if let Some(dir_string) = dirs.data_dir().to_str() {
			return Some(dir_string.to_string());
		}
	}
	None
}