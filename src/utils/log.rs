use colored::Colorize;

pub fn info(msg: &String) {
  println!("{}{}", "Info: ".bold().cyan(), msg);
}

pub fn error(msg: &String) {
  let red_wrapped = format!("Error: {}", msg);
  println!("{}", red_wrapped.bold().red());
}
