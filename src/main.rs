use std::{collections::HashMap, fs, path::{Path, PathBuf}};


const PATH: &'static str = "/home/naseef/Downloads_test";

fn main() {
    let path = Path::new(PATH);

    if path.exists() {
        if path.is_file() {
            return println!("The path exists but it is a file.");
        } else if path.is_dir() {
            println!("The path exists and it is a directory.");
        } else {
            return println!("The path exists but it is neither a file nor a directory.");
        }
    } else {
        return println!("The path does not exist.");
    }

    println!("Path: {:?}", path);
    println!("Can be continued...");

    let _category = HashMap::<&str, Vec<PathBuf>>::new();

    let dir_entries = fs::read_dir(path).unwrap();
    for entry in dir_entries {
        let entry = entry.unwrap();
        let path = entry.path();

        let hidden: bool = path.file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with(".");

        if path.is_dir() || hidden || path.is_symlink() {
            continue;
        }

        if let Some(extension) = path.extension() {
            let extension = extension.to_str().unwrap().to_lowercase();
            println!("File: {:?}, Extension: {}", path.file_name().unwrap(), extension);
        } else {
            println!("File: {:?}, Extension: None", path.file_name().unwrap());
        }
    }

}