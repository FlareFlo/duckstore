use std::fs;

use directories::BaseDirs;

#[allow(dead_code)]
/// This should be created from a constant to represent a persistent type of file and location
#[derive(Debug)]
pub struct StoreConfig<'a> {
	project_name: &'a str,
	folder: &'a str,
	file_name: &'a str,
	dir_type: &'a DirType,
}

// This path should be created statically at runtime
#[derive(Debug)]
pub struct StorePaths {
	pub base_prefix: String,
	pub project_name: String,
	pub sub_folder: String,
	pub file: String,
	pub constructed_path: String,
}

#[derive(Debug)]
pub enum DirType {
	/// Resolves to Roaming AppData or $HOME/.local/share
	Data,
	/// Resolves to Roaming AppData or $HOME/.config
	Config,
	/// Resolves to Local AppData or $HOME./cache
	Cache,
}

fn store(data: &[u8], store_config: &StoreConfig) -> Result<StorePaths, String> {
	if let Some(store_paths) = resolve_path(store_config) {

		// Result is dropped as it might already exists
		let _ = fs::create_dir_all(&format!("{}/{}/{}", store_paths.base_prefix, store_paths.project_name, store_paths.sub_folder));

		if let Err(error) = fs::write(&store_paths.constructed_path, data) {
			Err(format!("{}", error))
		} else {
			Ok(store_paths)
		}
	} else {
		Err(format!("Cannot resolve {:#?} to path", store_config))
	}
}

fn load(store_paths: &StorePaths) -> Result<Vec<u8>, String> {
	if let Ok(from_reader) = fs::read(&store_paths.constructed_path) {
		return Ok(from_reader);
	} else {
		Err(format!("Failed to read file"))
	}
}

fn resolve_path(store_config: &StoreConfig) -> Option<StorePaths> {
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
			return Some(StorePaths {
				base_prefix: base_str.to_string(),
				project_name: store_config.project_name.to_string(),
				sub_folder: store_config.folder.to_string(),
				file: store_config.file_name.to_string(),
				constructed_path: format!("{}/{}/{}/{}", base_str, store_config.project_name, store_config.folder, store_config.file_name),
			});
		}
	}
	None
}

#[cfg(test)]
mod test {
	use crate::{DirType, load, store, StoreConfig};

	#[test]
	fn yes() {
		const CFG: StoreConfig = StoreConfig {
			project_name: "duckstore",
			folder: "data",
			file_name: "data.bin",
			dir_type: &DirType::Data,
		};
		let result = store(b"Yes", &CFG).unwrap();

		let loaded = load(&result).unwrap();

		assert_eq!("Yes", String::from_utf8(loaded).unwrap())
	}
}