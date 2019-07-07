// Crates
extern crate indicatif;

// Use
use indicatif::ProgressBar;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

// Main
fn main() {
  let path = Path::new("E:\\download\\Brooklyn-s06");
  let all_dir_paths = dir_list(&path).unwrap();
  talk_a_walk(&path, &all_dir_paths).unwrap();
}

// List all folders in a directory
fn dir_list(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
  let mut all_dir_names = vec![];
  if dir.is_dir() {
    for entry in fs::read_dir(dir)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_dir() {
        all_dir_names.push(path);
      }
    }
  }
  Ok(all_dir_names)
}

// Go into each folder and move the files
fn talk_a_walk(copy_to_path: &Path, all_dir_paths: &Vec<PathBuf>) -> Result<(), io::Error> {
  if all_dir_paths.len() > 0 {
    // Create a progress bar
    let pbar_total = all_dir_paths.len() as u64;
    let pbar = ProgressBar::new(pbar_total);
    // Walk the directories and copy files
    for dir in all_dir_paths {
      // Increment for each folder
      pbar.inc(1);
      // Entry is a file in dir
      for entry in fs::read_dir(dir)? {
        let entry = entry?.path();
        let name = entry.file_name().unwrap();
        let formatted_path = PathBuf::from(copy_to_path.join(name));
        let new_path = Path::new(&formatted_path);
        // Check if the file is not already there
        if new_path.is_file() == false {
          // Copy every old path to its new path in main directory
          fs::copy(&entry, &new_path)?;
        } else {
          println!("File already there!: {:?}", name);
        }
      }
    }
    println!("Done!");
    pbar.finish();
  } else {
    println!("No directories!");
  }
  Ok(())
}
