use std::error::Error;
use std::fs;
use std::string::String;

struct Digit {
    index: usize,
    value: char,
}

fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_BACKTRACE", "1");

    let input: String = fs::read_to_string("testInput.txt")?;
    // let input: String = fs::read_to_string("input.txt")?;

    let result = process_input(input.split('\n').collect());
    println!("Result: {result}");

    Ok(())
}

fn process_input(input: Vec<&str>) -> u32 {
    let mut total: u32 = 0;
    for line in input {
        let mut first_digit = Digit {
            index: 0,
            value: '0',
        };
        let mut second_digit = Digit {
            index: 0,
            value: '0',
        };

        let chars = line.char_indices();
        let count = chars.clone().count();
        // First digit
        for character in chars.clone() {
            let current_index = character.0;
            let current_value = character.1;
            if current_index < count - 1 && current_value > first_digit.value {
                // Favors leftmost if there are multiple
                first_digit.index = current_index;
                first_digit.value = current_value;
            }
        }

        // Second digit
        for character in chars.clone() {
            let current_index = character.0;
            let current_value = character.1;
            if current_index > first_digit.index && current_value > second_digit.value {
                second_digit.index = current_index;
                second_digit.value = current_value;
            }
        }

        let mut bank_joltage = String::from("");
        // let bank_joltage: String;
        bank_joltage.push(first_digit.value);
        bank_joltage.push(second_digit.value);

        let bank_joltage = bank_joltage.parse::<u32>().unwrap();
        total += bank_joltage;
    }
    return total;
}
