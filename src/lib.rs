use std::fs;

use directories::BaseDirs;

/// This should be created from a constant to represent a persistent type of file and location
#[derive(Debug, Clone, Copy)]
pub struct PathConfig<'a> {
	pub project_prefix: &'a str,
	pub sub_folder: &'a str,
	pub file_name: &'a str,
	pub dir_type: &'a DirType,
}

// This path should be created statically at runtime initialization
#[derive(Debug, Clone)]
pub struct ResolvedPaths<'a> {
	pub config: PathConfig<'a>,
	pub base_path: String,
	pub constructed_path: String,
}

#[derive(Debug, Clone, Copy)]
pub enum DirType {
	/// Resolves to Roaming AppData or $HOME/.local/share
	Data,
	/// Resolves to Roaming AppData or $HOME/.config
	Config,
	/// Resolves to Local AppData or $HOME./cache
	Cache,
}

impl<'a> PathConfig<'a> {
	pub fn resolve(&self) -> Result<ResolvedPaths<'a>, String> {
		if let Some(dirs) = BaseDirs::new() {
			let base_dir;
			match self.dir_type {
				DirType::Data => {
					base_dir = dirs.data_dir();
				}
				DirType::Config => {
					base_dir = dirs.config_dir();
				}
				DirType::Cache => {
					base_dir = dirs.cache_dir();
				}
			}
			if let Some(base_str) = base_dir.to_str() {
				if let Err(err) = fs::create_dir_all(format!("{}/{}/{}", &base_str, &self.project_prefix, &self.sub_folder)) {
					return Err(format!("{}", err))
				}

				let config = *self;
				return Ok(ResolvedPaths {
					config,
					base_path: base_str.to_string(),
					constructed_path: format!("{}/{}/{}/{}", &base_str, self.project_prefix, self.sub_folder, self.file_name),
				});
			}
		}
		Err("Cannot resolve basic directories".to_owned())
	}
}

impl<'a> ResolvedPaths<'a> {
	pub fn store(&self, data: &[u8]) -> Result<(), String> {
		if let Err(error) = fs::write(&self.constructed_path, data) {
			Err(format!("{}", error))
		} else {
			Ok(())
		}
	}

	pub fn load(&self) -> Result<Vec<u8>, String> {
		if let Ok(from_reader) = fs::read(&self.constructed_path) {
			Ok(from_reader)
		} else {
			Err("Failed to read file".to_string())
		}
	}
}

#[cfg(test)]
mod test {
	use crate::{DirType, PathConfig};

	#[test]
	fn yes() {
		const CFG: PathConfig = PathConfig {
			project_prefix: "duckstore",
			sub_folder: "data",
			file_name: "data.bin",
			dir_type: &DirType::Data,
		};
		let resolved = &CFG.resolve().unwrap();

		resolved.store(b"Yes").unwrap();

		let loaded = resolved.load().unwrap();
		println!("{}", resolved.constructed_path);

		assert_eq!("Yes", String::from_utf8(loaded).unwrap())
	}
}