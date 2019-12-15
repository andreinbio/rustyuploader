extern crate zip;
extern crate walkdir;

use zip::{ZipWriter, write::FileOptions, CompressionMethod, result::ZipResult};
use walkdir::{WalkDir, DirEntry};

use std::io::{Write, Seek};
use std::path::Path;

use super::loader;

pub fn zip_dir(path: &str, name: &str, ignore_list: &Vec<String>) -> Vec<u8> {
    let walkdir = WalkDir::new(path.to_string());
    let mut it = walkdir.into_iter().filter_map(|e| e.ok());

    let dir_size: usize = it.nth(0).take().unwrap().metadata().unwrap().len() as usize;
    let dir_buffer: Vec<u8> = Vec::with_capacity(dir_size);
    let w = std::io::Cursor::new(dir_buffer);
    let mut zip = ZipWriter::new(w);
    let options = FileOptions::default().compression_method(CompressionMethod::Stored);

    // add root directory to zip
    zip.add_directory_from_path(Path::new(name), options).unwrap();
    let path_with_root_dir = path.replace(name, "");

    let zip_result = zip_dir_iterator(&mut it, zip, options, path_with_root_dir.as_str(), ignore_list);

    zip_result.unwrap().into_inner()
}

fn zip_dir_iterator<W: Write + Seek>(it: &mut dyn Iterator<Item=DirEntry>, mut zip: ZipWriter<W>, options: FileOptions, prefix: &str, ignore_list: &Vec<String>) -> ZipResult<W> {
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        if !ignore_list.is_empty() {
            let name_string = name.to_str().take().unwrap().replace("\\", "/");
            if ignore_list.into_iter().any(|path| name_string.contains(path)) {
                continue;
            }
        }

        if path.is_file() {
            zip.start_file_from_path(name, options).unwrap();
            zip.write_all(loader::read_file_bytes(path).as_slice());
        } else if !name.as_os_str().is_empty() {
            zip.add_directory_from_path(name, options).unwrap();
        }
    }

    zip.finish()
}