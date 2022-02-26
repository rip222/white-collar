mod error;
mod runner;
use error::AppError;
use std::fs::read_to_string;

fn main() -> Result<(), AppError> {
    if let Some(path) = std::env::args().nth(1) {
        let s = read_to_string(path)?;
        runner::run(&s);
    } else {
        let app_name = std::env::args().nth(0).unwrap();
        println!("Usage: {} <FILE>", app_name);
    }
    Ok(())
}
