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
        // println!("Range: {range}");
        let Some((start, end)) = range.split_once('-') else {
            println!("No hyphen found");
            return 69;
        };

        // Casting str to int, shadowing variables
        let start: u64 = start.trim().parse::<u64>().unwrap();
        let end: u64 = end.trim().parse::<u64>().unwrap();

        for value in start..=end {
            // Casting int to string, shadowing variables
            let value: String = value.to_string();
            // println!("\tValue: {value}");
            let len = value.len();
            // for pattern_len = len / 2; pattern_len >= 1; pattern_len -= 1 {
            for pattern_len in (1..=(len / 2)).rev() {
                if len % pattern_len == 0 {
                    let pattern_reps = len / pattern_len;
                    let pattern: &str = value.get(..pattern_len).unwrap();
                    // println!("\t\tPattern: {pattern}");
                    // println!("\t\tP_Length: {pattern_len}\tP_Reps: {pattern_reps}");

                    let matches_list: Vec<&str> = value.matches(pattern).collect();
                    // println!("\t\t\tVector: {:?}", matches_list);
                    if matches_list.len() == pattern_reps {
                        // println!("Adding {}", value.parse::<u64>().unwrap());
                        sum += value.parse::<u64>().unwrap();
                        break;
                    }
                }
            }
        }
    }
    return sum;
}
