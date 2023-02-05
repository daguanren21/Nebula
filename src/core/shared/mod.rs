pub mod ast;
pub mod compile_errors;

pub fn make_internal_err_str(str: &str) -> String {
  return format!("[Nebula Internal Error] {}", str);
}
