use std::io::stdin;

pub fn read_entry() -> String {
  let mut message = String::new();
  let reader = stdin();

  let reader_result = reader.read_line(&mut message);

  if reader_result.is_err() {
    println!("Error : {:?}", reader_result.err());
  }

  message.trim().to_string()
}

