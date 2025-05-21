use std::{
    fs::{self, File},
    io::Write,
};

use zip::{ZipWriter, write::SimpleFileOptions};

const ABS_RIR: &str = "/workspace/files";

fn seek_files_by_dir(dir: fs::ReadDir) -> Vec<std::path::PathBuf> {
    let mut files: Vec<std::path::PathBuf> = Vec::new();

    for entry in dir {
        let path: std::path::PathBuf = entry.unwrap().path();

        if path.is_file() {
            files.push(path);
        } else if path.is_dir() {
            let sub_dir = fs::read_dir(&path);
            match sub_dir {
                Ok(entries) => {
                    let sub_files = seek_files_by_dir(entries);
                    files.extend(sub_files);
                }
                Err(e) => {
                    eprintln!("Error reading directory: {}", e);
                }
            }
        }
    }

    return files;
}

fn main() {
    let dir = fs::read_dir(ABS_RIR);
    let zip_file = File::create("test.zip").unwrap();
    let mut zip = ZipWriter::new(zip_file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let dir = match dir {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return;
        }
    };

    let files = seek_files_by_dir(dir);

    for file in files {
        let relative_path = file.strip_prefix(ABS_RIR).unwrap().to_str().unwrap();

        zip.start_file(relative_path, options).unwrap();
        zip.write(b"hello").unwrap();
        zip.flush().unwrap();
    }

    zip.finish().unwrap();
}
