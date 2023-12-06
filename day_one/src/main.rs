use std::{
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
};

fn extract_numbers(text: String) -> Vec<u32> {
	const RADIX: u32 = 10;
	let mut numbers: Vec<u32> = Vec::new();
	for i in text.chars().flat_map(|x| x.to_digit(RADIX)) {
		numbers.push(i);
	}
	return numbers;
}

fn get_double_digits(numbers: Vec<u32>) -> u32 {
	let mut double_digit: Vec<u32> = Vec::new();
	double_digit.push(numbers.clone().into_iter().next().unwrap());
	double_digit.push(numbers.clone().into_iter().last().unwrap());
	double_digit.iter().fold(0, |acc, elem| acc * 10 + elem)
	// equal to:
	// let mut acc = 0;
	// for elem in vec {
	// 	acc *= 10;
	// 	acc += elem;
	// }
	// acc
}

fn sum(numbers: Vec<u32>) -> u32 {
	let mut result: u32 = 0;
	for n in numbers {
		result += n;
	}
	return result;
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main(){
	let lines = lines_from_file("./input.txt");
	let mut calibration_values: Vec<u32> = Vec::new();
	for line in lines {
		calibration_values.push(get_double_digits(extract_numbers(line)));
	}
	println!("{}", sum(calibration_values));
}