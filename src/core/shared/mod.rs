pub mod ast;
pub mod compile_errors;

pub fn nebula_interal_err(str: &str) -> String {
  return format!("[Nebula Internal Error] {}", str);
}
