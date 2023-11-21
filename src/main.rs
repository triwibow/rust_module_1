mod my_io;
fn main() {
	println!("Enter your phone number");
	let phone_number = my_io::read_entry();

	println!("Your phone number : {}" , phone_number);
}
