#![deny(clippy::all)]

use std::env;
use std::fs::File;
use std::io::Read;
use globset::{Glob, GlobSetBuilder};
use ignore::WalkBuilder;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn find_matching_files(patterns: Vec<String>, max_depth: Option<u32>, dir: Option<String>, ignore_dirs: Option<Vec<String>>) -> Vec<String> {
  let mut matching_files = Vec::new();
  let search_dir = dir.unwrap_or_else(|| env::current_dir().unwrap().to_string_lossy().into_owned());
  let mut builder = GlobSetBuilder::new();
  for pattern in patterns {
    builder.add(Glob::new(&pattern).unwrap());
  }
  let globset = builder.build().unwrap();
  let mut walk_builder = WalkBuilder::new(&search_dir);
  walk_builder.hidden(false).ignore(true).git_ignore(true).require_git(false);
  if let Some(depth) = max_depth {
    walk_builder.max_depth(Some(depth as usize));
  }
  if let Some(ignore_dirs) = ignore_dirs {
    for dir in ignore_dirs {
      walk_builder.add_ignore(dir);
    }
  }

  for result in walk_builder.build() {
    match result {
      Ok(entry) => {
        let path = entry.path();
        if path.is_file() {
          let relative_path = path.strip_prefix(&search_dir).unwrap_or(path);
          if globset.is_match(relative_path) {
            if let Some(path_str) = path.to_str() {
              matching_files.push(path_str.to_string());
            }
          }
        }
      }
      Err(e) => eprintln!("An error occurred while traversing the file: {}", e),
    }
  }
  matching_files
}

#[napi]
pub fn read_file_content(file_path: String) -> String {
  let supported_extensions = ["js", "jsx", "ts", "tsx", "rs"];
  let extension = std::path::Path::new(&file_path)
    .extension()
    .and_then(std::ffi::OsStr::to_str)
    .unwrap_or("");

  if !supported_extensions.contains(&extension) {
    return format!("Unsupported file type: {}", extension);
  }
  match File::open(&file_path) {
    Ok(mut file) => {
      let mut content = String::new();
      match file.read_to_string(&mut content) {
        Ok(_) => content,
        Err(e) => format!("An error occurred while reading the file content: {}", e),
      }
    },
    Err(e) => format!("An error occurred while opening the file: {}", e),
  }
}