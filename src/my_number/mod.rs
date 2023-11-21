pub fn string_to_number(text:String) -> i32 {
  text.parse::<i32>().unwrap()
}

pub fn is_odd_number(number:i32) -> bool {
  number % 2 == 0
}