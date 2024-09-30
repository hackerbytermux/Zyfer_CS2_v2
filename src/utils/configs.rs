use crate::settings::settings::Settings;


pub fn get_configs() -> Vec<std::path::PathBuf> {
    let config_files = std::fs::read_dir("./configs")
    .unwrap()
    .filter_map(|entry| {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.extension() == Some(std::ffi::OsStr::new("cfg")) {
            let path = path.file_name().unwrap();
            let path = std::path::PathBuf::from(path);
            Some(path)
        } else {
            None
        }
    })
    .collect::<Vec<_>>();

    config_files
}

pub fn save_config(name: &str, config: &Settings) {
    let path = std::path::Path::new("./configs").join(format!("{}.cfg", name));
    let serialized = serde_json::to_string(&config).unwrap();
    std::fs::write(path, serialized).unwrap();
}

pub fn load_config(name: &str) -> Settings {
    let path = std::path::Path::new("./configs").join(format!("{}.cfg", name));
    println!("Loading config: {}", path.display());
    let serialized = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&serialized).unwrap()
}

pub fn delete_config(name: &str) {
    let path = std::path::Path::new("./configs").join(format!("{}.cfg", name));
    std::fs::remove_file(path).unwrap();
}