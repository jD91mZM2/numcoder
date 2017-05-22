extern crate num;

use num::FromPrimitive;
use num::ToPrimitive;
use num::Zero;
use num::bigint::BigUint;

use std::env;
use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
	let mut stderr = io::stderr();

	let mut args = env::args();
	args.next(); // Ignore the executable name.

	let next = args.next();
	if next.is_none() {
		writeln!(stderr, "Usage: numcoder <encode/decode>").unwrap();
		return;
	}
	let next = next.unwrap();
	let next = next.as_str();

	match next {
		"encode" => {
			let input = get_input("Text: ");
			let mut num = BigUint::zero();

			for byte in input.trim().bytes().rev() {
				num = num + BigUint::from_u8(byte).unwrap(); // Will not fail
				num = num << 8;
			}

			println!("{}", num)
		},
		"decode" => {
			let input = get_input("Number: ");
			let num = BigUint::from_str(input.trim());
			if num.is_err() {
				println!("Not a number");
				return;
			}
			let mut num = num.unwrap();
			let zero = BigUint::zero();
			let max = BigUint::from_u16(std::u8::MAX as u16 + 1).unwrap(); // Will not fail

			let mut string = String::new();
			while num > zero {
				let shifted = num >> 8;
				num = shifted.clone();

				let byte = (shifted % max.clone()).to_u8();

				if byte.is_none() {
					writeln!(stderr, "Could not make BigInt into u8.").unwrap();
					return;
				}
				string.push(byte.unwrap() as char);
			}

			println!("{}", string)
		},
		_ => {
			writeln!(stderr, "Not a valid option").unwrap();
			return;
		},
	}
}

fn get_input(text: &str) -> String {
	print!("{}", text);
	io::stdout().flush().unwrap();

	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();

	input
}
