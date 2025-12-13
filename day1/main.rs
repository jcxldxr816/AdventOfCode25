use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_BACKTRACE", "1");

    // let input: String = fs::read_to_string("testInput.txt")?;
    let input: String = fs::read_to_string("input.txt")?;

    let result: i32 = process_input(50, input.split('\n').collect());
    println!("{}", result);

    Ok(())
}

fn process_input(starting_val: i32, input: Vec<&str>) -> i32 {
    let mut value: i32 = starting_val;
    let mut count: i32 = 0;

    for line in input {
        if line.len() < 2 {
            println!("Problem line: {line}");
            continue;
        }
        let (dir, amount) = line.split_at(1); //"L, 12"
        let adj_amount: i32 = amount.parse::<i32>().unwrap() % 100;
        if dir == "L" {
            value -= adj_amount;
            if value < 0 {
                value = 100 - (0 - value);
            }
        } else {
            value += adj_amount;
            if value > 99 {
                value = 0 + (value - 100);
            }
        }
        println!("Value: {value}");
        if value == 0 {
            count += 1;
        }
    }
    return count;
}
