use std::fs;

use directories::BaseDirs;

/// This should be created from a constant to represent a persistent type of file and location
#[derive(Debug, Clone, Copy)]
pub struct PathConfig<'a> {
	base_prefix: &'a str,
	sub_folder: &'a str,
	file_name: &'a str,
	dir_type: &'a DirType,
}

// This path should be created statically at runtime initialization
#[derive(Debug, Clone)]
pub struct ResolvedPaths<'a> {
	pub config: PathConfig<'a>,
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

pub fn store<'a>(data: &'a [u8], store_config: &'a PathConfig) -> Result<ResolvedPaths<'a>, String> {
	if let Some(store_paths) = resolve_path(store_config) {

		// Result is dropped as it might already exists
		let _ = fs::create_dir_all(&format!("{}/{}/{}", store_paths.config.base_prefix, store_paths.config.sub_folder, store_paths.config.file_name));

		if let Err(error) = fs::write(&store_paths.constructed_path, data) {
			Err(format!("{}", error))
		} else {
			Ok(store_paths)
		}
	} else {
		Err(format!("Cannot resolve {:#?} to path", store_config))
	}
}

pub fn load(store_paths: &ResolvedPaths) -> Result<Vec<u8>, String> {
	if let Ok(from_reader) = fs::read(&store_paths.constructed_path) {
		return Ok(from_reader);
	} else {
		Err(format!("Failed to read file"))
	}
}

fn resolve_path<'a>(store_config: &'a PathConfig) -> Option<ResolvedPaths<'a>> {
	if let Some(dirs) = BaseDirs::new() {
		let base_dir;
		match store_config.dir_type {
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
			let config = *store_config;
			return Some(ResolvedPaths {
				config,
				constructed_path: format!("{}/{}/{}/{}", base_str, store_config.base_prefix, store_config.sub_folder, store_config.file_name),
			});
		}
	}
	None
}

#[cfg(test)]
mod test {
	use crate::{DirType, load, store, PathConfig};

	#[test]
	fn yes() {
		const CFG: PathConfig = PathConfig {
			base_prefix: "duckstore",
			sub_folder: "data",
			file_name: "data.bin",
			dir_type: &DirType::Data,
		};
		let result = store(b"Yes", &CFG).unwrap();

		let loaded = load(&result).unwrap();
		println!("{}", result.constructed_path);

		assert_eq!("Yes", String::from_utf8(loaded).unwrap())
	}
}