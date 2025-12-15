use std::error::Error;
use std::fs;
use std::string::String;

#[derive(Clone, Copy)]
struct Digit {
    index: usize,
    value: char,
}

const BATTERIES: usize = 12;

fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_BACKTRACE", "1");

    // let input: String = fs::read_to_string("testInput.txt")?;
    let input: String = fs::read_to_string("input.txt")?;

    let result = process_input(input.split('\n').collect());
    println!("Result: {result}");

    Ok(())
}

fn process_input(input: Vec<&str>) -> u64 {
    let mut total: u64 = 0;
    for line in input {
        let chars = line.char_indices();

        //Init array with 12 Digit objects
        let mut digits = [Digit {
            index: 0,
            value: '0',
        }; BATTERIES];

        let mut bank_joltage = String::from("");
        let mut prev_index = 0;
        for (i, battery) in digits.iter_mut().enumerate() {
            let batteries_to_go = BATTERIES - i;

            for character in chars.clone() {
                let current_index = character.0;
                let current_value = character.1;

                // if current character is not to the right of last selected battery
                if i > 0 && current_index <= prev_index {
                    continue;
                }

                if current_index <= line.len() - batteries_to_go && current_value > battery.value {
                    battery.index = current_index;
                    battery.value = current_value;
                }
            }
            bank_joltage.push(battery.value);
            prev_index = battery.index;
        }
        let bank_joltage = bank_joltage.parse::<u64>().unwrap();
        total += bank_joltage;
    }
    return total;
}
