use crate::utils::{log, shared::return_and_print_err};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn get_env_args() -> Result<Vec<String>, String> {
  let args: Vec<String> = env::args().collect();
  if args.len() <= 1 {
    return_and_print_err("must give a target file".to_string())
  } else {
    log::info(&format!("Running args: {:?}", args));
    Ok(args)
  }
}

pub fn read_from_source(canonicalized: std::io::Result<PathBuf>) -> Result<String, String> {
  if let Ok(absolute_path) = canonicalized {
    if let Some(file_path) = absolute_path.to_str() {
      let file_path = file_path.to_string();
      log::info(&format!("Opening file: {}", file_path));
      match File::open(&file_path) {
        Err(why) => {
          return_and_print_err(format!("couldn't open {} ({})", file_path, why.to_string()))
        }
        Ok(mut file) => {
          let mut content = String::new();
          match file.read_to_string(&mut content) {
            Err(why) => {
              return_and_print_err(format!("couldn't read {} ({})", file_path, why.to_string()))
            }
            Ok(_) => Ok(content),
          }
        }
      }
    } else {
      return_and_print_err(String::from(
        "target file path is not a valid UTF-8 sequence",
      ))
    }
  } else {
    return_and_print_err(String::from("file path canonicalized failed"))
  }
}

pub fn compile_entry_file(_content: String) {
  log::info(&format!("Nebula Compiler {}", "v0.1"));
  // TODO: implement compilation
}

pub fn run() {
  if let Ok(args) = get_env_args() {
    // get_env_args ensures that index(1) is not None.
    let arg_file_path = args.get(1).unwrap();
    let target_path = Path::new(".").join(&arg_file_path);
    match read_from_source(target_path.canonicalize()) {
      Err(why) => log::error(&why),
      Ok(content) => {
        compile_entry_file(content)
        // TODO: more...
      }
    }
  }
}
