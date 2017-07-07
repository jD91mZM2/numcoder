extern crate num;

use num::FromPrimitive;
use num::ToPrimitive;
use num::Zero;
use num::bigint::BigUint;

use std::env;
use std::error::Error;
use std::io;
use std::io::Write;

fn main() {
	let stderr = io::stderr();
	let mut stderr = stderr.lock();

	macro_rules! attempt {
		($result:expr, $error:expr) => {
			match $result {
				Ok(something) => something,
				Err(_) => {
					writeln!(stderr, $error).unwrap();
					return;
				}
			}
		}
	}
	macro_rules! parse {
		($string:expr) => {
			attempt!($string.trim().parse(), "Invalid number")
		}
	}
	macro_rules! get_length {
		($limit:expr) => {
			attempt!(get_length($limit), "Binary representation of limit must be a one followed by one or more zeros")
		}
	}

	let mut args = env::args();
	args.next(); // Ignore the executable name.

	macro_rules! arg_or_ask {
		($question:expr) => {
			args.next().unwrap_or_else(|| get_input($question))
		}
	}

	let mode = args.next();
	if mode.is_none() {
		writeln!(
			stderr,
			"Usage: numcoder <encodestr/decodestr> [text]\n\
            Usage: numcoder <encode/decode> [comma separated numbers] [limit] [\"verbose\"]"
		).unwrap();
		return;
	}
	let mode = mode.unwrap();
	let mode = mode.as_str();

	match mode {
		"encode" => {
			let numbers = attempt!(
				parse_numbers(arg_or_ask!("Numbers: ")),
				"Invalid input. Expected comma separated list of numbers"
			);
			let limit = parse!(arg_or_ask!("Limit: "));
			let length = get_length!(limit);

			if let Some(result) = encode(&mut stderr, numbers.iter().map(|n| *n), limit, length) {
				println!("{}", result);
			}
		},
		"decode" => {
			let number = parse!(arg_or_ask!("Number: "));
			let limit = parse!(arg_or_ask!("Limit: "));

			let result = decode(number, limit, get_length!(limit));
			println!(
				"[{}]",
				result
					.iter()
					.map(|n| n.to_string())
					.collect::<Vec<String>>()
					.join(", ")
			);
		},
		"encodestr" => {
			let input = arg_or_ask!("Text: ");
			if let Some(result) = encode(
				&mut stderr,
				input.trim().as_bytes().iter().map(|n| *n as u32),
				256,
				8
			)
			{
				println!("{}", result);
			}
		},
		"decodestr" => {
			let input = arg_or_ask!("Number: ");
			let input = parse!(input);

			let result = decode(input, 256, 8);
			match String::from_utf8(result.iter().map(|n| *n as u8).collect()) {
				Ok(string) => println!("{}", string),
				Err(_) => writeln!(stderr, "Result is not valid UTF-8").unwrap(),
			}
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
fn parse_numbers(text: String) -> Result<Vec<u32>, Box<Error>> {
	let text = text.replace(char::is_whitespace, "");
	let mut text = text.as_str();
	if text.starts_with("[") {
		text = &text[1..];
	}
	if text.ends_with("]") {
		text = &text[..text.len() - 1];
	}

	let mut numbers = Vec::new();
	for token in text.split(",") {
		numbers.push(token.parse()?);
	}
	Ok(numbers)
}
fn get_length(mut limit: usize) -> Result<usize, ()> {
	if limit <= 1 {
		return Err(());
	}

	let mut length = 0;
	while (limit >> 1) > 0 {
		if limit % 2 != 0 {
			return Err(());
		}
		limit = limit >> 1;
		length += 1;
	}

	if limit != 1 {
		return Err(());
	}

	Ok(length)
}

fn encode<'a, I>(stderr: &mut io::StderrLock, numbers: I, limit: usize, length: usize) -> Option<BigUint>
where
	I: DoubleEndedIterator<Item = u32>,
{
	let mut result = BigUint::zero();

	for n in numbers.rev() {
		println!("{} >= {}", n, limit);
		if n >= limit as u32 {
			writeln!(
				stderr,
				"Limit less than or equals to one of the members in the array"
			).unwrap();
			return None;
		}
		result = result << length;
		result = result + BigUint::from_u32(n).unwrap();
	}

	Some(result)
}
fn decode(mut number: BigUint, limit: usize, length: usize) -> Vec<u32> {
	let bigzero = BigUint::zero();
	let biglimit = BigUint::from_usize(limit).unwrap();

	let mut result = Vec::new();
	while number > bigzero {
		let n = (&number % &biglimit).to_u32().unwrap();
		number = number >> length;

		result.push(n);
	}

	result
}
