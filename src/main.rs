use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

mod categories;

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

    let mut collection = HashMap::<&str, Vec<PathBuf>>::new();

    let dir_entries = fs::read_dir(path).unwrap();
    for entry in dir_entries {
        let entry = entry.unwrap();
        let path = entry.path();

        let hidden: bool = path.file_name().unwrap().to_str().unwrap().starts_with(".");

        if path.is_dir() || hidden || path.is_symlink() {
            continue;
        }

        let category = categories::get_category(path.extension());
        collection
            .entry(category)
            .or_insert_with(Vec::new)
            .push(path.clone());
    }

    for (category, files) in collection.iter() {
        println!("Category: {}", category);
        let dest = path.join(category);
        println!("Destination: {:?}", dest);
        fs::create_dir_all(&dest).expect("Failed to create directory.");
        for file in files {
            println!("  - {:?}", file.file_name().unwrap());
            let dest_file = dest.join(file.file_name().unwrap());
            fs::rename(file, dest_file).expect("Failed to move file.");
        }
    }
}
