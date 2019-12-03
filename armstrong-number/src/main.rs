fn main() {
    println!("{}",is_armstrong_number(9));
    println!("{}",is_armstrong_number(3));
    println!("{}",is_armstrong_number(10));
    println!("{}",is_armstrong_number(153));
    println!("{}",is_armstrong_number(190));
}

fn is_armstrong_number(number: i32) -> bool {
    let splitted_numbers: Vec<char> = number.to_string().chars().collect();
    let numbers = convert_to_integers(splitted_numbers);
    return add_armstrong_digits(numbers) == number;
}

fn add_armstrong_digits(digits: Vec<i32>) -> i32{
    let mut sum = 0;
    let power = digits.len() as u32;
    for num in digits {
        sum += num.pow(power);
    }
    return sum;
}

fn convert_to_integers(chars: Vec<char>) -> Vec<i32>{
    let mut numbers: Vec<i32> = Vec::new();
    for raw_number in chars {
        numbers.push(raw_number.to_digit(10).unwrap() as i32);
    }
    return numbers;
}

