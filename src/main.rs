// main.rs
use std::{
    collections::HashMap,
    fs,
    io,
    path::{Path, PathBuf},
};

mod categories;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let path = Path::new("/home/naseef/Downloads_test");

    // Validate path
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Path does not exist",
        ));
    }

    if !path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path is not a directory",
        ));
    }

    println!("Scanning directory: {}", path.display());

    // Phase 1: Collect and categorize
    let mut collection = HashMap::<&str, Vec<PathBuf>>::new();
    let category_map = categories::category_extension_map(); // Create once!

    let dir_entries = fs::read_dir(path)?;
    let mut scanned_count = 0;
    let mut skipped_count = 0;

    for entry_result in dir_entries {
        let entry = match entry_result {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Warning: Failed to read entry: {}", e);
                skipped_count += 1;
                continue;
            }
        };

        let file_path = entry.path();

        // Skip directories, symlinks, hidden files
        if file_path.is_dir() || file_path.is_symlink() {
            continue;
        }

        // Check for hidden files (safely)
        let is_hidden = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.starts_with('.'))
            .unwrap_or(false);

        if is_hidden {
            continue;
        }

        scanned_count += 1;

        // Get category
        let category = categories::get_category_with_map(
            file_path.extension(),
            &category_map,
        );

        collection
            .entry(category)
            .or_insert_with(Vec::new)
            .push(file_path);
    }

    println!("Scanned {} files", scanned_count);
    if skipped_count > 0 {
        println!("Skipped {} entries (errors)", skipped_count);
    }

    if collection.is_empty() {
        println!("No files to organize!");
        return Ok(());
    }

    println!("\nOrganizing into {} categories...\n", collection.len());

    // Phase 2: Create directories and move files
    let mut total_moved = 0;
    let mut total_failed = 0;

    for (&category, files) in collection.iter() {
        let dest_dir = path.join(category);

        // Create category directory
        if let Err(e) = fs::create_dir_all(&dest_dir) {
            eprintln!("Error: Failed to create directory {}: {}", category, e);
            total_failed += files.len();
            continue;
        }

        println!("Category: {} ({} files)", category, files.len());

        // Move files
        for file in files {
            let file_name = match file.file_name() {
                Some(name) => name,
                None => {
                    eprintln!("  ✗ Invalid filename: {:?}", file);
                    total_failed += 1;
                    continue;
                }
            };

            let dest_file = dest_dir.join(file_name);

            // Check if destination already exists
            if dest_file.exists() {
                eprintln!(
                    "  ⚠ Skipped {} (already exists)",
                    file_name.to_string_lossy()
                );
                total_failed += 1;
                continue;
            }

            // Move file
            match fs::rename(file, &dest_file) {
                Ok(_) => {
                    println!("  ✓ {}", file_name.to_string_lossy());
                    total_moved += 1;
                }
                Err(e) => {
                    eprintln!(
                        "  ✗ Failed to move {}: {}",
                        file_name.to_string_lossy(),
                        e
                    );
                    total_failed += 1;
                }
            }
        }
        println!();
    }

    // Summary
    println!("=================================");
    println!("Organization complete!");
    println!("  ✓ Moved: {} files", total_moved);
    if total_failed > 0 {
        println!("  ✗ Failed: {} files", total_failed);
    }
    println!("=================================");

    Ok(())
}