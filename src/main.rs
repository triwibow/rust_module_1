mod my_io;
mod my_number;

use my_number::conversion::{string_to_number, is_odd_number};
use my_number::type_check::type_of;

fn main() {
	println!("Enter number");
	let input_number = my_io::read_entry();
	println!("Your phone number : {}" , input_number);

	let number = string_to_number(input_number);
	let is_odd = is_odd_number(number);

	println!("is {number} is odd ? {is_odd}");

	println!("tipe of number is {}", type_of(&number));
	println!("tipe of number is {}", type_of(&is_odd));


}
