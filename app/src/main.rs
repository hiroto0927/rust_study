use std::{
    fs::{self, File},
    io::{BufReader, Read, Write},
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
            if let Ok(entries) = fs::read_dir(&path) {
                files.extend(seek_files_by_dir(entries));
            } else {
                eprintln!("Error reading directory: {:?}", path);
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
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return;
        }
    };

    let files = seek_files_by_dir(dir);
    let mut buffer: Vec<u8> = Vec::new();

    for file in files {
        let relative_path = file.strip_prefix(ABS_RIR).unwrap().to_str().unwrap();

        zip.start_file(relative_path, options).unwrap();
        let f = File::open(&file).unwrap();
        let mut reader = BufReader::new(&f);

        reader.read_to_end(&mut buffer).unwrap();
        zip.write_all(&buffer).unwrap();
        buffer.clear();
    }

    zip.finish().unwrap();
}
