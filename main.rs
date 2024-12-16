use std::fs::File;
use std::io::Write;
use rand::Rng;
// The 3 lines above are retrieving built in functions of rust
// the fn below is the function rust uses to handle it's errors.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // What this all is going to be doing is it will generate a random equation, and then put it in a text file. 
    // I would like to make this clear though, I used Visual Studio Code, and it would create the file on there, so I'm not sure if this actually works as intended
    // Continuing on, the let commands below are creating the random string. It can calculate up to 1 million, and currently it can't do negatives, but it doesn't give an error, but rather a 0 value. 
    // It can also do decimal positions.
    let mut rng = rand::thread_rng();
    let num1: u32 = rng.gen_range(1..=1000);
    let num2: u32 = rng.gen_range(1..=1000);
    let operator = match rng.gen_range(0..4) {
        0 => "+",
        1 => "-",
        2 => "*",
        _ => "/",
    };
// The code below just defines how it does the equation, as it is taking the output from above and formatting it into what we know as something like 2 + 2 = 4, or 15 * 20 = 300.
    let equation = match operator {
        "+" => format!("{} + {} = {}", num1, num2, num1 + num2),
        "-" => format!("{} - {} = {}", num1, num2, num1.saturating_sub(num2)),
        "*" => format!("{} * {} = {}", num1, num2, num1 * num2),
        "/" => {
            if num2 != 0 {
                format!("{} / {} = {:.2}", num1, num2, num1 as f64 / num2 as f64)
            } else {
                format!("{} / {} = undefined (division by zero)", num1, num2)
            }
        }
        _ => unreachable!(),
    };

    // Lastly, this is just writing the output from the equation into a txt file.
    let mut file = File::create("random_equation.txt")?;
    file.write_all(equation.as_bytes())?;

    println!("Equation has been written to random_equation.txt");

    Ok(())
}
