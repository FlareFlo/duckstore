use std::fs;

use directories::ProjectDirs;
use serde::{de::DeserializeOwned, Serialize};

fn store_local_data(data: &[u8], project_name: &str, file_path_name: &str) -> Result<(), String> {
	if let Some(dir_string) = resolve_path(project_name) {
		println!("{:?}", &dir_string);
		fs::create_dir(&dir_string.1).unwrap();
		if let Ok(serialized) = serde_json::to_vec(data) {
			if let Err(error) = fs::write(format!("{}\\{}", dir_string.1, file_path_name), serialized) {
				Err(format!("{}", error))
			} else {
				Ok(())
			}
		} else {
			Err(format!("Cannot parse struct"))
		}
	} else {
		Err(format!("Cannot resolve {project_name} to path"))
	}
}

fn load_local_data(project_name: &str, file_path_name: &str) -> Result<Vec<u8>, String> {
	if let Some(dir_string) = resolve_path(project_name) {
		if let Ok(from_reader) = fs::read(format!("{}/{}", dir_string.1, file_path_name)) {
			return Ok(from_reader);
		} else {
			Err(format!("Failed to read file"))
		}
	} else {
		Err(format!("Cannot resolve {} to path", project_name))
	}
}

fn resolve_path(project_name: &str) -> Option<(String, String)> {
	if let Some(dirs) = ProjectDirs::from("", "", project_name) {
		if let Some(dir_string) = dirs.data_dir().to_str() {
			return Some((dir_string.split(|c| c == '/' || c == '\\').collect::<Vec<&str>>().split_last().unwrap().1.join("/"), dir_string.to_string()));
		}
	}
	None
}

mod test {
	use std::io::{Bytes, Read};

	use directories::ProjectDirs;
	use serde::{Deserialize, Deserializer, Serialize, Serializer};

	use crate::store::store::{load_local_data, store_local_data};

	#[derive(serde::Deserialize, serde::Serialize, Default, Debug, Eq, PartialEq)]
	struct TestStruct {
		pub number: u32,
		pub string: String,
	}

	#[test]
	fn stuff() {
		let test_struct = TestStruct {
			number: 1,
			string: "string".to_string(),
		};
		store_local_data(&serde_json::to_vec(&test_struct).unwrap(), "duckstore", "cache.bin").unwrap();
		let retrieved = load_local_data("duckstore", "cache.bin").unwrap();

		assert_eq!(serde_json::from_slice::<TestStruct>(&retrieved).unwrap(), test_struct)
	}
}