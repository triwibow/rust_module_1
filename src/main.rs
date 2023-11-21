mod my_io;
mod my_number;
fn main() {
	println!("Enter number");
	let input_number = my_io::read_entry();
	println!("Your phone number : {}" , input_number);

	let number = my_number::string_to_number(input_number);
	let is_odd = my_number::is_odd_number(number);

	println!("is {number} is odd ? {is_odd}");

}
