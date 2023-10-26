use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main(){
	println!("Adivina el numero!!");

	let secret_number = rand::thread_rng().gen_range(1..=100);

	loop {
		println!("Intenta adivinar el numero: ");

		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Fallo al leer la linea");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Mas grande!\n"),
			Ordering::Greater => println!("Mas chico!\n"),
			Ordering::Equal => {
				println!("Ganaste!\n");
				break;
			}
		}

	}

}