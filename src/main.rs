use std::ffi::OsString;
use std::fs::read_dir;
use std::io;
use std::path::Path;

// Main
fn main() {
  let path: &Path = Path::new("E:\\download\\Veep S01");
  let all_dir_names = dir_list(&path);
  println!("Name: {:?}", all_dir_names);
}

// List all files in a directory
fn dir_list(dir: &Path) -> io::Result<Vec<OsString>> {
  let mut all_dir_names = vec![];
  if dir.is_dir() {
    for entry in read_dir(dir)? {
      let entry = entry?;
      let name = entry.file_name();
      all_dir_names.push(name);
    }
  }
  Ok(all_dir_names)
}

// Get all media files and subtitles files
