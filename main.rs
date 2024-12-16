use std::fs::File;
use std::io::Write;
use rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a random equation
    let mut rng = rand::thread_rng();
    let num1: u32 = rng.gen_range(1..=1000);
    let num2: u32 = rng.gen_range(1..=1000);
    let operator = match rng.gen_range(0..4) {
        0 => "+",
        1 => "-",
        2 => "*",
        _ => "/",
    };

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

    // Write the equation to a text file
    let mut file = File::create("random_equation.txt")?;
    file.write_all(equation.as_bytes())?;

    println!("Equation has been written to random_equation.txt");

    Ok(())
}