use std::env;
use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use std::fs::Metadata;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::write;

use git2::Repository;
use walkdir::WalkDir;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("You must provide the URL of a GitHub repository.");
        return;
    }

    let repo_url = &args[1];
    let exclusions = &args[2..];
    let local_path = "temp_repo";

    let mut included_files = Vec::new();
    let mut excluded_files = Vec::new();

    match Repository::clone(repo_url, local_path) {
        Err(e) => eprintln!("Failed to clone repository: {}", e),
        Ok(_) => {
            let mut file_contents = HashMap::new();

            for entry in WalkDir::new(local_path) {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(e) => {
                        eprintln!("Error while walking through the repository: {}", e);
                        continue;
                    }
                };

                if entry.file_type().is_file() {
                    let path = entry.path().display().to_string();
                    if path.contains(".git"){continue;}

                    let excluded = exclusions.iter().any(|exclusion| {
                        if exclusion.starts_with("type:") {
                            let filetype = &exclusion[5..];
                            Path::new(&path).extension() == Some(filetype.as_ref())
                        } else if exclusion.starts_with("dir:") {
                            let dir = &exclusion[4..];
                            path.contains(dir)
                        } else {
                            false
                        }
                    });

                    if excluded {
                        excluded_files.push(path.clone());
                        continue;
                    }

                    let file = match File::open(&path) {
                        Ok(file) => file,
                        Err(e) => {
                            eprintln!("Failed to open file: {}", e);
                            continue;
                        }
                    };

                    let metadata = match file.metadata() {
                        Ok(metadata) => metadata,
                        Err(e) => {
                            eprintln!("Failed to get file metadata: {}", e);
                            continue;
                        }
                    };

                    let mut buf_reader = BufReader::new(file);
                    let mut contents = String::new();

                    match buf_reader.read_to_string(&mut contents) {
                        Ok(_) => {
                            file_contents.insert(path.clone(), contents);
                            included_files.push((path, metadata.len()));
                        },
                        Err(e) => {
                            eprintln!("Failed to read file:{}\n  File path:{}", e, path);
                            continue;
                        }
                    }
                }
            }

            let output = file_contents.iter()
                .map(|(path, contents)| format!("{}\n\n{}", path, contents))
                .collect::<Vec<_>>()
                .join("\n\n");

            let included_files_output = included_files.iter()
                .map(|(path, size)| format!("{}: {:.2} KB", path, *size as f64 / 1024.0))
                .collect::<Vec<_>>()
                .join("\n");

            let excluded_files_output = excluded_files.join("\n");

            if let Err(e) = write("output.txt", output) {
                eprintln!("Failed to write to output.txt: {}", e);
            }

            if let Err(e) = write("files.txt", included_files_output) {
                eprintln!("Failed to write to files.txt: {}", e);
            }

            if let Err(e) = write("excluded.txt", excluded_files_output) {
                eprintln!("Failed to write to excluded.txt: {}", e);
            }

            if let Err(e) = std::fs::remove_dir_all(local_path) {
                eprintln!("Failed to remove temporary repository directory: {}", e);
            }
        }
    }
}

