use std::cmp;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_BACKTRACE", "1");

    // let input: String = fs::read_to_string("testInput.txt")?;
    let input: String = fs::read_to_string("input.txt")?;

    let result = process_input(input.trim().split('\n').collect());
    println!("Result: {result}");

    Ok(())
}

fn process_input(lines: Vec<&str>) -> u32 {
    let rows = lines.len();
    let columns = lines[0].trim().len();
    let mut diagram = vec![vec![false; columns]; rows];

    let mut total = 0;

    // Finding all rolls
    for line_index in 0..rows {
        let line = lines[line_index].trim();
        // Looping through each char in the line
        for (index, value) in line.char_indices() {
            if value == '@' {
                diagram[index][line_index] = true;
            }
        }
    }

    for y in 0..rows {
        for x in 0..columns {
            if diagram[x][y] == true {
                let mut count = 0;
                // println!("Roll at {x},{y}");
                for check_x in x.saturating_sub(1)..=cmp::min(x + 1, columns - 1) {
                    for check_y in y.saturating_sub(1)..=cmp::min(y + 1, rows - 1) {
                        // println!("\tChecking X: {check_x}\tChecking Y: {check_y}");
                        if diagram[check_x][check_y] == true {
                            count += 1;
                        }
                    }
                }
                count -= 1; //subtracting the center roll
                if count < 4 {
                    total += 1;
                }
            }
        }
    }

    return total;
}
