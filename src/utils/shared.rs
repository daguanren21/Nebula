pub fn return_and_print_err<T>(msg: String) -> Result<T, String> {
  crate::utils::log::error(&msg);
  Err(msg)
}

#[macro_export]
macro_rules! hashmap {
  ($( $key: expr => $val: expr ),*) => {{
    let mut map = ::std::collections::HashMap::new();
    $( map.insert($key, $val); )*
    map
  }}
}
