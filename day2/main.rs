use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_BACKTRACE", "1");

    // let input: String = fs::read_to_string("testInput.txt")?;
    let input: String = fs::read_to_string("input.txt")?;

    let result = process_input(input.split(',').collect());
    println!("Result: {result}");

    Ok(())
}

fn process_input(input: Vec<&str>) -> u64 {
    let mut sum: u64 = 0;

    for range in input {
        let Some((start, end)) = range.split_once('-') else {
            println!("No hyphen found");
            return 0;
        };
        // println!("\nStart: {start}\nEnd:{end}");

        // Casting str to int, shadowing variables
        let start: u64 = start.trim().parse::<u64>().unwrap();
        let end: u64 = end.trim().parse::<u64>().unwrap();

        for value in start..=end {
            // Casting int to string, shadowing variables
            let value: String = value.to_string();

            let len = value.len();
            if len % 2 == 0 {
                //only considering even length IDs
                let (first_half, second_half) = value.split_at(len / 2);
                if first_half == second_half {
                    sum += value.parse::<u64>().unwrap();
                    // println!("Match: {first_half} {second_half}");
                }
            }
        }
    }
    return sum;
}
