use std::fs;
use directories::ProjectDirs;

pub trait Store<'a>: serde::Serialize + serde::Deserialize<'a> {
	fn store_local_data(&self, project_name: &str) -> Result<(), String> {
		if let Some(dirs) = ProjectDirs::from("", "", project_name) {
			if let Ok(json) = serde_json::to_string(self) {
				if let Some(string_dir) = dirs.data_dir().to_str() {
					if let Err(error) = fs::write(string_dir, json) {
						Err(format!("{}", error))
					} else {
						Ok(())
					}
				} else {
					Err(format!("Failed to resolve path into string"))
				}
			} else {
				Err(format!("Failed to resolve path {} to string", project_name))
			}
		} else {
			Err(format!("Failed to resolve {} OS path", project_name))
		}
	}
	fn load_local_data(project_name: &str) -> Result<Self, String> {

				if let Ok(from_reader) = fs::read(dir_string) {
					return if let Ok(parsed) = serde_json::from_slice(&from_reader) {
						Ok(parsed)
					} else {
						Err(format!("Failed to parse struct"))
					};
				} else {
					Err(format!("Failed to read file"))
				}

	}
}

fn resolve_path(project_name: &str) -> Option<String> {
	if let Some(dirs) = ProjectDirs::from("", "", project_name) {
		if let Some(dir_string) = dirs.data_dir().to_str() {
			Some(dir_string.to_string())
		} else {
			Err(format!("Failed to resolve path {} to string", project_name))
		}
	} else {
		Err(format!("Failed to resolve {} to OS path", project_name))
	}
}