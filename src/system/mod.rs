use std::path::PathBuf;

#[cfg(target_family = "windows")]
pub fn load_executable() -> Vec<PathBuf> {
    let mut executables = vec![];
    let bin_paths = env!("Path")
        .split(";")
        .map(String::from)
        .collect::<Vec<String>>();

    let executable_suffix_list = env!("PATHEXT")
        .split(";")
        .map(String::from)
        .collect::<Vec<String>>();

    for path in bin_paths {
        let p = PathBuf::from(&path);
        match p.read_dir() {
            Ok(d) => {
                for result in d.into_iter() { match result {
                    Ok(entry) => { if let Ok(file_type) = entry.file_type() {
                        let os_file_name = entry.file_name();
                        let file_name = os_file_name.to_str().unwrap();
                        if file_type.is_file() &&
                            executable_suffix_list.iter().any(|s| file_name.ends_with(&s.to_ascii_lowercase()))
                        {
                            executables.push(entry.path());
                        }
                    }},
                    _ => ()
                }}
            },
            _ => ()
        }
    }

    executables
}